use actix::prelude::*;
use bitcoin::{Block, Transaction, BlockHash, Txid};
use serde::{Deserialize, Serialize};

pub mod storage;
pub mod network;
pub mod mempool;
pub mod chain;

// Storage Actor Messages
#[derive(Message)]
#[rtype(result = "Result<(), crate::error::StorageError>")]
pub struct StoreBlock {
    pub block: Block,
}

#[derive(Message)]
#[rtype(result = "Result<Option<Block>, crate::error::StorageError>")]
pub struct GetBlock {
    pub hash: BlockHash,
}

#[derive(Message)]
#[rtype(result = "Result<(), crate::error::StorageError>")]
pub struct AddTransaction {
    pub tx: Transaction,
}

#[derive(Message)]
#[rtype(result = "Result<Option<Transaction>, crate::error::StorageError>")]
pub struct GetTransaction {
    pub txid: Txid,
}

// Network Actor Messages
#[derive(Message)]
#[rtype(result = "Result<(), crate::error::NetworkError>")]
pub struct NewPeer {
    pub peer_id: String,
    pub address: String,
    pub user_agent: Option<String>,
}

#[derive(Message)]
#[rtype(result = "Result<(), crate::error::NetworkError>")]
pub struct DisconnectPeer {
    pub peer_id: String,
    pub reason: String,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<PeerInfo>, crate::error::NetworkError>")]
pub struct GetPeers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub user_agent: Option<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Message)]
#[rtype(result = "Result<(), crate::error::NetworkError>")]
pub struct BroadcastTransaction {
    pub tx: Transaction,
}

#[derive(Message)]
#[rtype(result = "Result<(), crate::error::NetworkError>")]
pub struct BroadcastBlock {
    pub block: Block,
}

// Chain Actor Messages
#[derive(Message)]
#[rtype(result = "Result<ChainInfo, crate::error::StorageError>")]
pub struct GetChainInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub best_block_hash: String,
    pub difficulty: f64,
    pub median_time: u64,
    pub verification_progress: f64,
    pub initial_block_download: bool,
    pub chain_work: String,
    pub size_on_disk: u64,
    pub pruned: bool,
}

// Mempool Actor Messages
#[derive(Message)]
#[rtype(result = "Result<(), crate::error::StorageError>")]
pub struct AddToMempool {
    pub tx: Transaction,
    pub fee: u64,
    pub fee_rate: f64,
}

#[derive(Message)]
#[rtype(result = "Result<Option<Transaction>, crate::error::StorageError>")]
pub struct GetFromMempool {
    pub txid: Txid,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Txid>, crate::error::StorageError>")]
pub struct GetMempoolTxids;

#[derive(Message)]
#[rtype(result = "Result<MempoolInfo, crate::error::StorageError>")]
pub struct GetMempoolInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub max_mempool: u64,
    pub mempool_min_fee: f64,
    pub min_relay_tx_fee: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_info_serialization() {
        let peer_info = PeerInfo {
            id: "peer1".to_string(),
            address: "127.0.0.1:8333".to_string(),
            user_agent: Some("/Satoshi:0.21.0/".to_string()),
            connected_at: chrono::Utc::now(),
            bytes_sent: 1024,
            bytes_received: 2048,
        };

        let json = serde_json::to_string(&peer_info).unwrap();
        let deserialized: PeerInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(peer_info.id, deserialized.id);
        assert_eq!(peer_info.address, deserialized.address);
        assert_eq!(peer_info.user_agent, deserialized.user_agent);
        assert_eq!(peer_info.bytes_sent, deserialized.bytes_sent);
        assert_eq!(peer_info.bytes_received, deserialized.bytes_received);
    }

    #[test]
    fn test_chain_info_serialization() {
        let chain_info = ChainInfo {
            chain: "regtest".to_string(),
            blocks: 100,
            headers: 100,
            best_block_hash: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f".to_string(),
            difficulty: 1.0,
            median_time: 1234567890,
            verification_progress: 1.0,
            initial_block_download: false,
            chain_work: "0000000000000000000000000000000000000000000000000000000000000064".to_string(),
            size_on_disk: 1024000,
            pruned: false,
        };

        let json = serde_json::to_string(&chain_info).unwrap();
        let deserialized: ChainInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(chain_info.chain, deserialized.chain);
        assert_eq!(chain_info.blocks, deserialized.blocks);
        assert_eq!(chain_info.headers, deserialized.headers);
        assert_eq!(chain_info.best_block_hash, deserialized.best_block_hash);
        assert_eq!(chain_info.difficulty, deserialized.difficulty);
    }

    #[test]
    fn test_mempool_info_serialization() {
        let mempool_info = MempoolInfo {
            size: 50,
            bytes: 25000,
            usage: 25000,
            max_mempool: 300000000,
            mempool_min_fee: 0.00001000,
            min_relay_tx_fee: 0.00001000,
        };

        let json = serde_json::to_string(&mempool_info).unwrap();
        let deserialized: MempoolInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(mempool_info.size, deserialized.size);
        assert_eq!(mempool_info.bytes, deserialized.bytes);
        assert_eq!(mempool_info.usage, deserialized.usage);
        assert_eq!(mempool_info.max_mempool, deserialized.max_mempool);
        assert_eq!(mempool_info.mempool_min_fee, deserialized.mempool_min_fee);
        assert_eq!(mempool_info.min_relay_tx_fee, deserialized.min_relay_tx_fee);
    }
}
