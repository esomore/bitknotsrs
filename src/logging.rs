use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::{global, trace::TraceError};
use opentelemetry_jaeger::new_agent_pipeline;
use std::fs::OpenOptions;
use std::io;

use crate::config::{LoggingConfig, LogFormat, OpenTelemetryConfig};
use crate::error::{NodeError, NodeResult};

pub fn init(config: &LoggingConfig) -> NodeResult<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let mut layers = Vec::new();

    // Console layer
    match config.format {
        LogFormat::Json => {
            let console_layer = fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_current_span(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .boxed();
            layers.push(console_layer);
        }
        LogFormat::Pretty => {
            let console_layer = fmt::layer()
                .pretty()
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .boxed();
            layers.push(console_layer);
        }
        LogFormat::Compact => {
            let console_layer = fmt::layer()
                .compact()
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .boxed();
            layers.push(console_layer);
        }
    }

    // File layer
    if config.file_enabled {
        if let Some(file_path) = &config.file_path {
            // Create log directory if it doesn't exist
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)?;

            let file_layer = fmt::layer()
                .json()
                .with_writer(file)
                .with_span_events(FmtSpan::CLOSE)
                .with_current_span(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .boxed();

            layers.push(file_layer);
        }
    }

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(layers);

    registry.init();

    tracing::info!("Logging initialized");
    tracing::info!("Log level: {}", config.level);
    tracing::info!("Log format: {:?}", config.format);

    if config.file_enabled {
        if let Some(file_path) = &config.file_path {
            tracing::info!("File logging enabled: {:?}", file_path);
        }
    }

    Ok(())
}

pub fn init_opentelemetry(config: &OpenTelemetryConfig) -> Result<(), TraceError> {
    if !config.enabled {
        return Err(TraceError::Other("OpenTelemetry disabled".into()));
    }

    // Simplified OpenTelemetry initialization
    tracing::info!("OpenTelemetry would be initialized here");
    tracing::info!("Service name: {}", config.service_name);
    tracing::info!("Service version: {}", config.service_version);

    if let Some(endpoint) = &config.endpoint {
        tracing::info!("OTEL endpoint: {}", endpoint);
    }

    Ok(())
}

pub fn shutdown_opentelemetry() {
    global::shutdown_tracer_provider();
    tracing::info!("OpenTelemetry shutdown complete");
}

// Structured logging macros for common Bitcoin node events
#[macro_export]
macro_rules! log_block_event {
    ($level:ident, $block_hash:expr, $height:expr, $message:expr) => {
        tracing::$level!(
            block_hash = %$block_hash,
            height = $height,
            event_type = "block",
            "{}", $message
        );
    };
}

#[macro_export]
macro_rules! log_tx_event {
    ($level:ident, $txid:expr, $fee:expr, $message:expr) => {
        tracing::$level!(
            txid = %$txid,
            fee = $fee,
            event_type = "transaction",
            "{}", $message
        );
    };
}

#[macro_export]
macro_rules! log_peer_event {
    ($level:ident, $peer_id:expr, $address:expr, $message:expr) => {
        tracing::$level!(
            peer_id = %$peer_id,
            address = %$address,
            event_type = "peer",
            "{}", $message
        );
    };
}

#[macro_export]
macro_rules! log_storage_event {
    ($level:ident, $operation:expr, $key:expr, $message:expr) => {
        tracing::$level!(
            operation = %$operation,
            key = %$key,
            event_type = "storage",
            "{}", $message
        );
    };
}

// Performance timing utilities
pub struct Timer {
    start: std::time::Instant,
    name: String,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        tracing::debug!("Starting timer: {}", name);
        Self {
            start: std::time::Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        tracing::debug!(
            timer = %self.name,
            duration_ms = duration.as_millis(),
            "Timer completed"
        );
    }
}

#[macro_export]
macro_rules! time_operation {
    ($name:expr, $block:block) => {{
        let _timer = $crate::logging::Timer::new($name);
        $block
    }};
}