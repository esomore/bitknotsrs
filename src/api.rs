use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::config::Config;
use crate::events::EventManager;
use crate::error::ApiResult;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub network: String,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct NodeInfoResponse {
    pub version: String,
    pub network: String,
    pub chain_height: Option<u64>,
    pub peer_count: u64,
    pub mempool_size: u64,
    pub storage_size_mb: f64,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub blocks_processed: u64,
    pub transactions_processed: u64,
    pub peers_connected: u64,
    pub uptime_seconds: u64,
    pub memory_usage_mb: f64,
}

pub async fn health() -> ActixResult<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        network: "regtest".to_string(), // TODO: Get from config
        uptime_seconds: 0, // TODO: Calculate actual uptime
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn node_info(
    _config: web::Data<Config>,
) -> ActixResult<HttpResponse> {
    // TODO: Get actual data from actors
    let response = NodeInfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        network: "regtest".to_string(),
        chain_height: Some(0),
        peer_count: 0,
        mempool_size: 0,
        storage_size_mb: 0.0,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn stats() -> ActixResult<HttpResponse> {
    // TODO: Get actual metrics
    let response = StatsResponse {
        blocks_processed: 0,
        transactions_processed: 0,
        peers_connected: 0,
        uptime_seconds: 0,
        memory_usage_mb: 0.0,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn peers() -> ActixResult<HttpResponse> {
    // TODO: Get actual peer list from network actor
    let peers: Vec<serde_json::Value> = vec![];
    Ok(HttpResponse::Ok().json(peers))
}

pub async fn mempool() -> ActixResult<HttpResponse> {
    // TODO: Get actual mempool data
    let mempool_info = serde_json::json!({
        "size": 0,
        "bytes": 0,
        "usage": 0,
        "max_mempool": 300000000,
        "mempool_min_fee": 0.00001000,
        "min_relay_tx_fee": 0.00001000
    });

    Ok(HttpResponse::Ok().json(mempool_info))
}

#[derive(Deserialize)]
pub struct GetBlockQuery {
    pub hash: Option<String>,
    pub height: Option<u64>,
}

pub async fn get_block(query: web::Query<GetBlockQuery>) -> ActixResult<HttpResponse> {
    if query.hash.is_none() && query.height.is_none() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Either hash or height parameter is required"
        })));
    }

    // TODO: Get actual block data from storage actor
    let block_info = serde_json::json!({
        "hash": "0000000000000000000000000000000000000000000000000000000000000000",
        "height": 0,
        "time": 0,
        "nonce": 0,
        "difficulty": 1.0,
        "tx": []
    });

    Ok(HttpResponse::Ok().json(block_info))
}

#[derive(Deserialize)]
pub struct GetTransactionQuery {
    pub txid: String,
}

pub async fn get_transaction(query: web::Query<GetTransactionQuery>) -> ActixResult<HttpResponse> {
    // TODO: Get actual transaction data from storage actor
    let tx_info = serde_json::json!({
        "txid": query.txid,
        "size": 0,
        "vsize": 0,
        "weight": 0,
        "fee": 0,
        "vin": [],
        "vout": []
    });

    Ok(HttpResponse::Ok().json(tx_info))
}

#[derive(Deserialize)]
pub struct SendRawTransactionRequest {
    pub hex: String,
}

pub async fn send_raw_transaction(
    req: web::Json<SendRawTransactionRequest>,
    _event_manager: web::Data<EventManager>,
) -> ActixResult<HttpResponse> {
    // TODO: Validate and broadcast transaction
    info!("Received raw transaction: {}", req.hex);

    // For now, return a dummy txid
    let response = serde_json::json!({
        "txid": "0000000000000000000000000000000000000000000000000000000000000000"
    });

    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/info", web::get().to(node_info))
            .route("/stats", web::get().to(stats))
            .route("/peers", web::get().to(peers))
            .route("/mempool", web::get().to(mempool))
            .route("/block", web::get().to(get_block))
            .route("/transaction", web::get().to(get_transaction))
            .route("/sendrawtransaction", web::post().to(send_raw_transaction))
    );
}