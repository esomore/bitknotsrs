use actix::prelude::*;
use tracing::{info, error};

use crate::config::Config;
use crate::error::StorageError;
use super::{StoreBlock, GetChainInfo, ChainInfo};

pub struct ChainActor {
    _storage_actor: Addr<super::storage::StorageActor>,
}

impl ChainActor {
    pub fn new(config: &Config, storage_actor: Addr<super::storage::StorageActor>) -> Self {
        info!("Chain actor initialized");
        Self {
            _storage_actor: storage_actor,
        }
    }
}

impl Actor for ChainActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Chain actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Chain actor stopped");
    }
}

impl Handler<StoreBlock> for ChainActor {
    type Result = Result<(), StorageError>;

    fn handle(&mut self, msg: StoreBlock, _ctx: &mut Self::Context) -> Self::Result {
        info!("Processing new block: {}", msg.block.block_hash());
        // TODO: Validate block and update chain state
        Ok(())
    }
}

impl Handler<GetChainInfo> for ChainActor {
    type Result = Result<ChainInfo, StorageError>;

    fn handle(&mut self, _msg: GetChainInfo, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: Return actual chain information
        Ok(ChainInfo {
            chain: "regtest".to_string(),
            blocks: 0,
            headers: 0,
            best_block_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            difficulty: 1.0,
            median_time: 0,
            verification_progress: 1.0,
            initial_block_download: false,
            chain_work: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            size_on_disk: 0,
            pruned: false,
        })
    }
}