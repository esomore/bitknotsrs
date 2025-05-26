use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use crate::error::{ConfigError, ConfigResult};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub network: Network,
    pub datadir: PathBuf,
    pub api: ApiConfig,
    pub rpc: RpcConfig,
    pub storage: StorageConfig,
    pub network_config: NetworkConfig,
    pub metrics: MetricsConfig,
    pub logging: LoggingConfig,
    pub events: EventsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Network {
    #[serde(rename = "mainnet")]
    Mainnet,
    #[serde(rename = "testnet")]
    Testnet,
    #[serde(rename = "regtest")]
    Regtest,
}

impl FromStr for Network {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Mainnet),
            "testnet" => Ok(Network::Testnet),
            "regtest" => Ok(Network::Regtest),
            _ => Err(ConfigError::InvalidNetwork(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub enabled: bool,
    pub cors_enabled: bool,
    pub rate_limit: Option<RateLimitConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcConfig {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub enabled: bool,
    pub allowed_methods: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    pub rocks_db_path: PathBuf,
    pub cache_size: usize,
    pub max_open_files: i32,
    pub compression: CompressionType,
    pub backup_enabled: bool,
    pub backup_interval_hours: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CompressionType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "zstd")]
    Zstd,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub listen_port: u16,
    pub max_peers: usize,
    pub connection_timeout_secs: u64,
    pub discovery_interval_secs: u64,
    pub custom_peers: Vec<String>,
    pub enable_dns_seeds: bool,
    pub enable_peer_exchange: bool,
    pub zmq: ZmqConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZmqConfig {
    pub enabled: bool,
    pub pub_port: Option<u16>,
    pub sub_endpoints: Vec<String>,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub otel: OpenTelemetryConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenTelemetryConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub service_name: String,
    pub service_version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub file_enabled: bool,
    pub file_path: Option<PathBuf>,
    pub max_file_size_mb: u64,
    pub max_files: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "compact")]
    Compact,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventsConfig {
    pub enabled_publishers: Vec<String>,
    pub zmq: ZmqEventConfig,
    pub k8s: K8sEventConfig,
    pub webhook: WebhookEventConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZmqEventConfig {
    pub enabled: bool,
    pub port: u16,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct K8sEventConfig {
    pub enabled: bool,
    pub namespace: String,
    pub node_name: String,
    pub event_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookEventConfig {
    pub enabled: bool,
    pub endpoints: Vec<String>,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
}

impl Config {
    pub fn load(path: &str) -> ConfigResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::InvalidValue {
                field: "config_file".to_string(),
                value: format!("Cannot read {}: {}", path, e),
            })?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::InvalidValue {
                field: "config_format".to_string(),
                value: format!("Invalid TOML: {}", e),
            })?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> ConfigResult<()> {
        // Validate ports are not conflicting
        let mut ports = vec![self.api.port, self.rpc.port, self.metrics.port];
        if let Some(zmq_port) = self.network_config.zmq.pub_port {
            ports.push(zmq_port);
        }
        ports.push(self.events.zmq.port);

        ports.sort();
        for window in ports.windows(2) {
            if window[0] == window[1] {
                return Err(ConfigError::InvalidValue {
                    field: "ports".to_string(),
                    value: format!("Port {} is used multiple times", window[0]),
                });
            }
        }

        // Validate data directory
        if !self.datadir.exists() {
            std::fs::create_dir_all(&self.datadir)
                .map_err(|e| ConfigError::InvalidValue {
                    field: "datadir".to_string(),
                    value: format!("Cannot create directory: {}", e),
                })?;
        }

        Ok(())
    }

    /// Get the effective listen port for the current network
    pub fn effective_listen_port(&self) -> u16 {
        // Use configured port if set, otherwise use network default
        if self.network_config.listen_port != 0 {
            self.network_config.listen_port
        } else {
            // Return network-specific default ports
            match self.network {
                Network::Mainnet => 8333,
                Network::Testnet => 18333,
                Network::Regtest => 18444,
            }
        }
    }

    /// Check if DNS seed discovery should be enabled for this network
    pub fn should_use_dns_seeds(&self) -> bool {
        // DNS seeds are only useful for mainnet and testnet, not regtest
        self.network_config.enable_dns_seeds && matches!(self.network, Network::Mainnet | Network::Testnet)
    }

    /// Get custom peers combined with network-specific localhost peers
    pub fn all_custom_peers(&self) -> Vec<String> {
        let mut peers = self.network_config.custom_peers.clone();

        // Add localhost peers for regtest
        if matches!(self.network, Network::Regtest) {
            peers.push(format!("127.0.0.1:{}", self.effective_listen_port()));
            peers.push(format!("localhost:{}", self.effective_listen_port()));
        }

        peers
    }

    pub fn default_regtest() -> Self {
        Self {
            network: Network::Regtest,
            datadir: PathBuf::from("./data"),
            api: ApiConfig {
                host: "127.0.0.1".to_string(),
                port: 8332,
                enabled: true,
                cors_enabled: true,
                rate_limit: Some(RateLimitConfig {
                    requests_per_minute: 100,
                    burst_size: 10,
                }),
            },
            rpc: RpcConfig {
                host: "127.0.0.1".to_string(),
                port: 18443,
                user: Some("user".to_string()),
                password: Some("pass".to_string()),
                enabled: true,
                allowed_methods: vec![
                    "getblockchaininfo".to_string(),
                    "getbestblockhash".to_string(),
                    "getblock".to_string(),
                    "gettransaction".to_string(),
                    "sendrawtransaction".to_string(),
                ],
            },
            storage: StorageConfig {
                rocks_db_path: PathBuf::from("./data/rocksdb"),
                cache_size: 1024 * 1024 * 256, // 256MB
                max_open_files: 1000,
                compression: CompressionType::Lz4,
                backup_enabled: false,
                backup_interval_hours: 24,
            },
            network_config: NetworkConfig {
                listen_port: 18444,
                max_peers: 8,
                connection_timeout_secs: 30,
                discovery_interval_secs: 60,
                custom_peers: vec![],
                enable_dns_seeds: true,
                enable_peer_exchange: true,
                zmq: ZmqConfig {
                    enabled: true,
                    pub_port: Some(28332),
                    sub_endpoints: vec![],
                    topics: vec![
                        "hashblock".to_string(),
                        "hashtx".to_string(),
                        "rawblock".to_string(),
                        "rawtx".to_string(),
                    ],
                },
            },
            metrics: MetricsConfig {
                enabled: true,
                host: "127.0.0.1".to_string(),
                port: 9090,
                path: "/metrics".to_string(),
                otel: OpenTelemetryConfig {
                    enabled: false,
                    endpoint: None,
                    service_name: "bitknotsrs".to_string(),
                    service_version: env!("CARGO_PKG_VERSION").to_string(),
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: LogFormat::Json,
                file_enabled: true,
                file_path: Some(PathBuf::from("./logs/bitknotsrs.log")),
                max_file_size_mb: 100,
                max_files: 10,
            },
            events: EventsConfig {
                enabled_publishers: vec!["zmq".to_string()], // Disable k8s for tests
                zmq: ZmqEventConfig {
                    enabled: true,
                    port: 28333,
                    topics: vec![
                        "block".to_string(),
                        "transaction".to_string(),
                        "peer".to_string(),
                    ],
                },
                k8s: K8sEventConfig {
                    enabled: false, // Disable for tests
                    namespace: "bitcoin".to_string(),
                    node_name: "bitknotsrs-node".to_string(),
                    event_types: vec![
                        "block".to_string(),
                        "transaction".to_string(),
                        "peer".to_string(),
                        "chain".to_string(),
                    ],
                },
                webhook: WebhookEventConfig {
                    enabled: false,
                    endpoints: vec![],
                    timeout_secs: 10,
                    retry_attempts: 3,
                },
            },
        }
    }

    pub fn test_config() -> Self {
        let mut config = Self::default_regtest();
        config.datadir = PathBuf::from("./test_data");
        config.storage.rocks_db_path = PathBuf::from("./test_data/rocksdb");
        config.logging.file_enabled = false; // Disable file logging for tests
        config.events.enabled_publishers = vec![]; // Disable all event publishers for tests
        config.events.zmq.enabled = false;
        config.events.k8s.enabled = false;
        config.events.webhook.enabled = false;
        config.metrics.enabled = false; // Disable metrics for tests
        config.rpc.enabled = false; // Disable RPC for tests
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_listen_port() {
        let mut config = Config::default_regtest();

        // Test with configured port
        config.network_config.listen_port = 9999;
        assert_eq!(config.effective_listen_port(), 9999);

        // Test with default port (0 means use network default)
        config.network_config.listen_port = 0;
        assert_eq!(config.effective_listen_port(), 18444); // regtest default

        // Test different networks
        config.network = Network::Mainnet;
        assert_eq!(config.effective_listen_port(), 8333);

        config.network = Network::Testnet;
        assert_eq!(config.effective_listen_port(), 18333);
    }

    #[test]
    fn test_should_use_dns_seeds() {
        let mut config = Config::default_regtest();

        // Regtest should not use DNS seeds even if enabled
        config.network_config.enable_dns_seeds = true;
        assert!(!config.should_use_dns_seeds());

        // Mainnet should use DNS seeds if enabled
        config.network = Network::Mainnet;
        config.network_config.enable_dns_seeds = true;
        assert!(config.should_use_dns_seeds());

        // Mainnet should not use DNS seeds if disabled
        config.network_config.enable_dns_seeds = false;
        assert!(!config.should_use_dns_seeds());

        // Testnet should use DNS seeds if enabled
        config.network = Network::Testnet;
        config.network_config.enable_dns_seeds = true;
        assert!(config.should_use_dns_seeds());
    }

    #[test]
    fn test_all_custom_peers() {
        let mut config = Config::default_regtest();
        config.network_config.custom_peers = vec!["peer1:8333".to_string(), "peer2:8333".to_string()];

        let peers = config.all_custom_peers();

        // Should include custom peers
        assert!(peers.contains(&"peer1:8333".to_string()));
        assert!(peers.contains(&"peer2:8333".to_string()));

        // Should include localhost peers for regtest
        assert!(peers.contains(&"127.0.0.1:18444".to_string()));
        assert!(peers.contains(&"localhost:18444".to_string()));

        // Mainnet should not include localhost peers
        config.network = Network::Mainnet;
        let mainnet_peers = config.all_custom_peers();
        assert!(mainnet_peers.contains(&"peer1:8333".to_string()));
        assert!(mainnet_peers.contains(&"peer2:8333".to_string()));
        assert!(!mainnet_peers.contains(&"127.0.0.1:18444".to_string()));
        assert!(!mainnet_peers.contains(&"localhost:18444".to_string()));
    }
}
