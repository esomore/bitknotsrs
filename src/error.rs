use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("RPC error: {0}")]
    Rpc(#[from] RpcError),

    #[error("API error: {0}")]
    Api(#[from] ApiError),

    #[error("Metrics error: {0}")]
    Metrics(#[from] MetricsError),

    #[error("Event publishing error: {0}")]
    Events(#[from] EventError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::de::Error),

    #[error("Serialization error: {0}")]
    SerializationSer(#[from] toml::ser::Error),

    #[error("Actor mailbox error: {0}")]
    Mailbox(#[from] actix::MailboxError),

    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid network: {0}")]
    InvalidNetwork(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),

    #[error("Database not found: {path}")]
    DatabaseNotFound { path: String },

    #[error("Corruption detected in {component}")]
    Corruption { component: String },

    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection failed to {peer}: {reason}")]
    ConnectionFailed { peer: String, reason: String },

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Peer disconnected: {peer}")]
    PeerDisconnected { peer: String },

    #[error("ZMQ error: {0}")]
    Zmq(String),
}

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Invalid method: {0}")]
    InvalidMethod(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Authentication failed")]
    AuthenticationFailed,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Failed to initialize metrics: {0}")]
    Initialization(String),

    #[error("Failed to export metrics: {0}")]
    Export(String),

    #[error("Invalid metric name: {0}")]
    InvalidName(String),
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to publish event: {0}")]
    PublishFailed(String),

    #[error("Publisher not available: {0}")]
    PublisherUnavailable(String),

    #[error("Kubernetes API error: {0}")]
    KubernetesApi(String),

    #[error("Event serialization error: {0}")]
    Serialization(String),
}

// Result type aliases for convenience
pub type NodeResult<T> = Result<T, NodeError>;
pub type ConfigResult<T> = Result<T, ConfigError>;
pub type StorageResult<T> = Result<T, StorageError>;
pub type NetworkResult<T> = Result<T, NetworkError>;
pub type RpcResult<T> = Result<T, RpcError>;
pub type ApiResult<T> = Result<T, ApiError>;
pub type MetricsResult<T> = Result<T, MetricsError>;
pub type EventResult<T> = Result<T, EventError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::InvalidNetwork("invalid".to_string());
        assert_eq!(error.to_string(), "Invalid network: invalid");

        let error = ConfigError::MissingField("port".to_string());
        assert_eq!(error.to_string(), "Missing required field: port");

        let error = ConfigError::InvalidValue {
            field: "timeout".to_string(),
            value: "negative".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid value for timeout: negative");
    }

    #[test]
    fn test_storage_error_display() {
        let error = StorageError::DatabaseNotFound {
            path: "/tmp/db".to_string(),
        };
        assert_eq!(error.to_string(), "Database not found: /tmp/db");

        let error = StorageError::Corruption {
            component: "blocks".to_string(),
        };
        assert_eq!(error.to_string(), "Corruption detected in blocks");

        let error = StorageError::Serialization("invalid data".to_string());
        assert_eq!(error.to_string(), "Serialization error: invalid data");
    }

    #[test]
    fn test_network_error_display() {
        let error = NetworkError::ConnectionFailed {
            peer: "127.0.0.1:8333".to_string(),
            reason: "timeout".to_string(),
        };
        assert_eq!(error.to_string(), "Connection failed to 127.0.0.1:8333: timeout");

        let error = NetworkError::Protocol("invalid message".to_string());
        assert_eq!(error.to_string(), "Protocol error: invalid message");

        let error = NetworkError::PeerDisconnected {
            peer: "peer1".to_string(),
        };
        assert_eq!(error.to_string(), "Peer disconnected: peer1");
    }

    #[test]
    fn test_rpc_error_display() {
        let error = RpcError::InvalidMethod("unknown".to_string());
        assert_eq!(error.to_string(), "Invalid method: unknown");

        let error = RpcError::InvalidParams("missing txid".to_string());
        assert_eq!(error.to_string(), "Invalid parameters: missing txid");

        let error = RpcError::AuthenticationFailed;
        assert_eq!(error.to_string(), "Authentication failed");
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::InvalidRequest("malformed JSON".to_string());
        assert_eq!(error.to_string(), "Invalid request: malformed JSON");

        let error = ApiError::NotFound("block not found".to_string());
        assert_eq!(error.to_string(), "Not found: block not found");

        let error = ApiError::Internal("database error".to_string());
        assert_eq!(error.to_string(), "Internal server error: database error");
    }

    #[test]
    fn test_metrics_error_display() {
        let error = MetricsError::Initialization("failed to start".to_string());
        assert_eq!(error.to_string(), "Failed to initialize metrics: failed to start");

        let error = MetricsError::Export("connection refused".to_string());
        assert_eq!(error.to_string(), "Failed to export metrics: connection refused");

        let error = MetricsError::InvalidName("invalid-name".to_string());
        assert_eq!(error.to_string(), "Invalid metric name: invalid-name");
    }

    #[test]
    fn test_event_error_display() {
        let error = EventError::PublishFailed("network error".to_string());
        assert_eq!(error.to_string(), "Failed to publish event: network error");

        let error = EventError::PublisherUnavailable("zmq".to_string());
        assert_eq!(error.to_string(), "Publisher not available: zmq");

        let error = EventError::Serialization("invalid JSON".to_string());
        assert_eq!(error.to_string(), "Event serialization error: invalid JSON");
    }

    #[test]
    fn test_node_error_from_conversions() {
        let config_error = ConfigError::InvalidNetwork("test".to_string());
        let node_error: NodeError = config_error.into();
        assert!(matches!(node_error, NodeError::Config(_)));

        let storage_error = StorageError::Serialization("test".to_string());
        let node_error: NodeError = storage_error.into();
        assert!(matches!(node_error, NodeError::Storage(_)));
    }
}