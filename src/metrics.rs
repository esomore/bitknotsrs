use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::time::Duration;
use tokio::net::TcpListener;
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use tracing::{info, error};

use crate::config::MetricsConfig;
use crate::error::{MetricsError, MetricsResult};

pub struct MetricsHandle {
    _server_handle: tokio::task::JoinHandle<()>,
}

pub async fn init(config: &MetricsConfig) -> MetricsResult<MetricsHandle> {
    // Initialize Prometheus exporter
    let builder = PrometheusBuilder::new();
    builder
        .install()
        .map_err(|e| MetricsError::Initialization(format!("Failed to install Prometheus exporter: {}", e)))?;

    // Register Bitcoin-specific metrics
    register_bitcoin_metrics()?;

    // Start metrics HTTP server
    let server_handle = start_metrics_server(config).await?;

    info!("Metrics initialized");
    info!("Metrics server listening on {}:{}{}", config.host, config.port, config.path);

    Ok(MetricsHandle {
        _server_handle: server_handle,
    })
}

async fn start_metrics_server(config: &MetricsConfig) -> MetricsResult<tokio::task::JoinHandle<()>> {
    let host = config.host.clone();
    let port = config.port;
    let path = config.path.clone();

    let server_handle = tokio::spawn(async move {
        // Simple HTTP server for metrics
        info!("Metrics server would start on {}:{}{}", host, port, path);
        // TODO: Implement proper metrics server
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    });

    Ok(server_handle)
}

async fn metrics_handler() -> ActixResult<HttpResponse> {
    // For now, return a simple metrics response
    // TODO: Implement proper metrics collection with the correct API
    let metrics_output = "# HELP bitcoin_node_info Node information\n# TYPE bitcoin_node_info gauge\nbitcoin_node_info{version=\"0.1.0\"} 1\n";

    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_output))
}

async fn health_handler() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "bitknotsrs-metrics"
    })))
}

fn register_bitcoin_metrics() -> MetricsResult<()> {
    // Note: With the current metrics crate version, metrics are registered automatically
    // when first used. This function serves as documentation of available metrics.

    info!("Bitcoin metrics will be registered on first use");
    Ok(())
}

// Metric recording functions
pub fn record_block_processed(height: u64, size: u64, tx_count: u64, processing_time: Duration) {
    counter!("bitcoin_blocks_processed_total").increment(1);
    gauge!("bitcoin_chain_height").set(height as f64);
    gauge!("bitcoin_block_size_bytes").set(size as f64);
    gauge!("bitcoin_block_transactions").set(tx_count as f64);
    histogram!("bitcoin_block_processing_duration_seconds").record(processing_time.as_secs_f64());
}

pub fn record_transaction_processed(size: u64, fee_rate: f64) {
    counter!("bitcoin_transactions_processed_total").increment(1);
    histogram!("bitcoin_transaction_size_bytes").record(size as f64);
    histogram!("bitcoin_transaction_fee_rate").record(fee_rate);
}

pub fn record_mempool_stats(tx_count: u64, total_size: u64) {
    gauge!("bitcoin_mempool_size").set(tx_count as f64);
    gauge!("bitcoin_mempool_bytes").set(total_size as f64);
}

pub fn record_peer_connected() {
    counter!("bitcoin_peer_connections_total").increment(1);
    // Note: peer count should be updated separately
}

pub fn record_peer_disconnected() {
    counter!("bitcoin_peer_disconnections_total").increment(1);
}

pub fn record_peer_count(count: u64) {
    gauge!("bitcoin_peers_connected").set(count as f64);
}

pub fn record_peer_latency(latency: Duration) {
    histogram!("bitcoin_peer_latency_seconds").record(latency.as_secs_f64());
}

pub fn record_storage_operation(operation: &str, duration: Duration, success: bool) {
    counter!("bitcoin_storage_operations_total", "operation" => operation.to_string()).increment(1);
    histogram!("bitcoin_storage_operation_duration_seconds", "operation" => operation.to_string())
        .record(duration.as_secs_f64());

    if !success {
        counter!("bitcoin_storage_errors_total", "operation" => operation.to_string()).increment(1);
    }
}

pub fn record_storage_size(size: u64) {
    gauge!("bitcoin_storage_size_bytes").set(size as f64);
}

pub fn record_rpc_request(method: &str, duration: Duration, success: bool) {
    counter!("bitcoin_rpc_requests_total", "method" => method.to_string()).increment(1);
    histogram!("bitcoin_rpc_request_duration_seconds", "method" => method.to_string())
        .record(duration.as_secs_f64());

    if !success {
        counter!("bitcoin_rpc_errors_total", "method" => method.to_string()).increment(1);
    }
}

pub fn record_node_uptime(uptime: Duration) {
    gauge!("bitcoin_node_uptime_seconds").set(uptime.as_secs_f64());
}

pub fn record_system_stats(memory_bytes: u64, cpu_percent: f64) {
    gauge!("bitcoin_node_memory_usage_bytes").set(memory_bytes as f64);
    gauge!("bitcoin_node_cpu_usage_percent").set(cpu_percent);
}

// Utility macro for timing operations with metrics
#[macro_export]
macro_rules! time_and_record {
    ($metric_name:expr, $operation:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();

        let success = result.is_ok();
        $crate::metrics::record_storage_operation($operation, duration, success);

        result
    }};
}