use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::config::{Config, EventsConfig};
use crate::error::{EventError, EventResult};

#[derive(Clone)]
pub struct EventManager {
    publishers: Arc<RwLock<Vec<Box<dyn EventPublisher + Send + Sync>>>>,
}

#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &BitcoinEvent) -> EventResult<()>;
    fn name(&self) -> &str;
    fn is_enabled(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: BitcoinEventType,
    pub network: String,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum BitcoinEventType {
    BlockAdded {
        hash: String,
        height: u64,
        size: u64,
        tx_count: u64,
        timestamp: u64,
    },
    TransactionAdded {
        txid: String,
        size: u64,
        fee: u64,
        fee_rate: f64,
    },
    PeerConnected {
        peer_id: String,
        address: String,
        user_agent: Option<String>,
    },
    PeerDisconnected {
        peer_id: String,
        address: String,
        reason: String,
    },
    ChainReorg {
        old_tip: String,
        new_tip: String,
        depth: u64,
    },
    MempoolUpdate {
        tx_count: u64,
        total_size: u64,
        min_fee_rate: f64,
        max_fee_rate: f64,
    },
    SyncProgress {
        current_height: u64,
        target_height: u64,
        progress_percent: f64,
    },
    NodeStarted {
        version: String,
        network: String,
        data_dir: String,
    },
    NodeStopping {
        reason: String,
        uptime_seconds: u64,
    },
}

impl EventManager {
    pub async fn new(config: &Config) -> EventResult<Self> {
        let mut publishers: Vec<Box<dyn EventPublisher + Send + Sync>> = Vec::new();

        // Initialize ZMQ publisher (disabled due to thread safety issues)
        // TODO: Fix ZMQ thread safety issues
        // if config.events.enabled_publishers.contains(&"zmq".to_string()) && config.events.zmq.enabled {
        //     let zmq_publisher = ZmqEventPublisher::new(&config.events.zmq).await?;
        //     publishers.push(Box::new(zmq_publisher));
        // }

        // Initialize Kubernetes publisher (disabled for simplicity)
        // TODO: Re-enable when needed
        // if config.events.enabled_publishers.contains(&"k8s".to_string()) && config.events.k8s.enabled {
        //     let k8s_publisher = K8sEventPublisher::new(&config.events.k8s).await?;
        //     publishers.push(Box::new(k8s_publisher));
        // }

        // Initialize Webhook publisher
        if config.events.enabled_publishers.contains(&"webhook".to_string()) && config.events.webhook.enabled {
            let webhook_publisher = WebhookEventPublisher::new(&config.events.webhook).await?;
            publishers.push(Box::new(webhook_publisher));
        }

        info!("Event manager initialized with {} publishers", publishers.len());

        Ok(Self {
            publishers: Arc::new(RwLock::new(publishers)),
        })
    }

    pub async fn publish(&self, event_type: BitcoinEventType, network: &str, node_id: &str) -> EventResult<()> {
        let event = BitcoinEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            network: network.to_string(),
            node_id: node_id.to_string(),
        };

        let publishers = self.publishers.read().await;
        let mut errors = Vec::new();

        for publisher in publishers.iter() {
            if publisher.is_enabled() {
                if let Err(e) = publisher.publish(&event).await {
                    error!("Failed to publish event via {}: {}", publisher.name(), e);
                    errors.push(e);
                } else {
                    info!("Event published via {}: {}", publisher.name(), event.id);
                }
            }
        }

        if !errors.is_empty() && errors.len() == publishers.len() {
            return Err(EventError::PublishFailed(format!(
                "All publishers failed: {:?}", errors
            )));
        }

        Ok(())
    }
}

// ZMQ Event Publisher (disabled due to thread safety issues)
// TODO: Implement proper thread-safe ZMQ wrapper

// Kubernetes Event Publisher
pub struct K8sEventPublisher {
    client: kube::Client,
    events_api: kube::Api<k8s_openapi::api::core::v1::Event>,
    namespace: String,
    node_name: String,
    event_types: Vec<String>,
    enabled: bool,
}

impl K8sEventPublisher {
    pub async fn new(config: &crate::config::K8sEventConfig) -> EventResult<Self> {
        let client = kube::Client::try_default().await
            .map_err(|e| EventError::KubernetesApi(e.to_string()))?;
        let events_api = kube::Api::namespaced(client.clone(), &config.namespace);

        info!("K8s event publisher initialized for namespace: {}", config.namespace);

        Ok(Self {
            client,
            events_api,
            namespace: config.namespace.clone(),
            node_name: config.node_name.clone(),
            event_types: config.event_types.clone(),
            enabled: config.enabled,
        })
    }
}

#[async_trait::async_trait]
impl EventPublisher for K8sEventPublisher {
    async fn publish(&self, event: &BitcoinEvent) -> EventResult<()> {
        use k8s_openapi::api::core::v1::Event;
        use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
        use k8s_openapi::api::core::v1::ObjectReference;
        use kube::api::PostParams;

        let event_type = match &event.event_type {
            BitcoinEventType::BlockAdded { .. } => "block",
            BitcoinEventType::TransactionAdded { .. } => "transaction",
            BitcoinEventType::PeerConnected { .. } | BitcoinEventType::PeerDisconnected { .. } => "peer",
            BitcoinEventType::ChainReorg { .. } => "chain",
            _ => "general",
        };

        if !self.event_types.contains(&event_type.to_string()) {
            return Ok(()); // Skip if event type not configured
        }

        let (reason, message) = match &event.event_type {
            BitcoinEventType::BlockAdded { hash, height, .. } => {
                ("NewBlock".to_string(), format!("New block {} at height {}", hash, height))
            }
            BitcoinEventType::TransactionAdded { txid, fee, .. } => {
                ("NewTransaction".to_string(), format!("New transaction {} with fee {}", txid, fee))
            }
            BitcoinEventType::PeerConnected { peer_id, address, .. } => {
                ("PeerConnected".to_string(), format!("Peer {} connected from {}", peer_id, address))
            }
            BitcoinEventType::PeerDisconnected { peer_id, reason, .. } => {
                ("PeerDisconnected".to_string(), format!("Peer {} disconnected: {}", peer_id, reason))
            }
            BitcoinEventType::ChainReorg { old_tip, new_tip, depth } => {
                ("ChainReorg".to_string(), format!("Chain reorg from {} to {} (depth: {})", old_tip, new_tip, depth))
            }
            _ => ("BitcoinEvent".to_string(), "Bitcoin node event".to_string()),
        };

        let k8s_event = Event {
            metadata: ObjectMeta {
                name: Some(format!("bitknotsrs-{}", event.id)),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some(self.node_name.clone()),
                namespace: Some(self.namespace.clone()),
                ..Default::default()
            },
            reason: Some(reason),
            message: Some(message),
            type_: Some("Normal".to_string()),
            action: Some(format!("Bitcoin{}", event_type)),
            first_timestamp: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::Time(event.timestamp)),
            last_timestamp: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::Time(event.timestamp)),
            count: Some(1),
            ..Default::default()
        };

        self.events_api.create(&PostParams::default(), &k8s_event).await
            .map_err(|e| EventError::KubernetesApi(e.to_string()))?;
        Ok(())
    }

    fn name(&self) -> &str {
        "kubernetes"
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// Webhook Event Publisher
pub struct WebhookEventPublisher {
    client: reqwest::Client,
    endpoints: Vec<String>,
    timeout: std::time::Duration,
    retry_attempts: u32,
    enabled: bool,
}

impl WebhookEventPublisher {
    pub async fn new(config: &crate::config::WebhookEventConfig) -> EventResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| EventError::PublishFailed(e.to_string()))?;

        info!("Webhook event publisher initialized with {} endpoints", config.endpoints.len());

        Ok(Self {
            client,
            endpoints: config.endpoints.clone(),
            timeout: std::time::Duration::from_secs(config.timeout_secs),
            retry_attempts: config.retry_attempts,
            enabled: config.enabled,
        })
    }
}

#[async_trait::async_trait]
impl EventPublisher for WebhookEventPublisher {
    async fn publish(&self, event: &BitcoinEvent) -> EventResult<()> {
        let payload = serde_json::to_string(event)
            .map_err(|e| EventError::Serialization(e.to_string()))?;

        for endpoint in &self.endpoints {
            let mut attempts = 0;
            let mut success = false;

            while attempts <= self.retry_attempts && !success {
                match self.client
                    .post(endpoint)
                    .header("Content-Type", "application/json")
                    .body(payload.clone())
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            success = true;
                        } else {
                            warn!("Webhook {} returned status: {}", endpoint, response.status());
                        }
                    }
                    Err(e) => {
                        warn!("Failed to send webhook to {}: {}", endpoint, e);
                    }
                }

                attempts += 1;
                if !success && attempts <= self.retry_attempts {
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * attempts as u64)).await;
                }
            }

            if !success {
                error!("Failed to deliver webhook to {} after {} attempts", endpoint, self.retry_attempts + 1);
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "webhook"
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}