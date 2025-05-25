use actix::prelude::*;
use tracing::{info, error};

use crate::config::Config;
use crate::error::StorageError;
use super::{AddToMempool, GetFromMempool, GetMempoolTxids, GetMempoolInfo, MempoolInfo};

pub struct MempoolActor {
    _storage_actor: Addr<super::storage::StorageActor>,
}

impl MempoolActor {
    pub fn new(config: &Config, storage_actor: Addr<super::storage::StorageActor>) -> Self {
        info!("Mempool actor initialized");
        Self {
            _storage_actor: storage_actor,
        }
    }
}

impl Actor for MempoolActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Mempool actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Mempool actor stopped");
    }
}

impl Handler<AddToMempool> for MempoolActor {
    type Result = Result<(), StorageError>;

    fn handle(&mut self, msg: AddToMempool, _ctx: &mut Self::Context) -> Self::Result {
        info!("Adding transaction to mempool: {} (fee: {}, fee_rate: {})",
               msg.tx.txid(), msg.fee, msg.fee_rate);
        // TODO: Validate transaction and add to mempool
        Ok(())
    }
}

impl Handler<GetFromMempool> for MempoolActor {
    type Result = Result<Option<bitcoin::Transaction>, StorageError>;

    fn handle(&mut self, msg: GetFromMempool, _ctx: &mut Self::Context) -> Self::Result {
        info!("Getting transaction from mempool: {}", msg.txid);
        // TODO: Get actual transaction from mempool
        Ok(None)
    }
}

impl Handler<GetMempoolTxids> for MempoolActor {
    type Result = Result<Vec<bitcoin::Txid>, StorageError>;

    fn handle(&mut self, _msg: GetMempoolTxids, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: Return actual mempool transaction IDs
        Ok(vec![])
    }
}

impl Handler<GetMempoolInfo> for MempoolActor {
    type Result = Result<MempoolInfo, StorageError>;

    fn handle(&mut self, _msg: GetMempoolInfo, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: Return actual mempool information
        Ok(MempoolInfo {
            size: 0,
            bytes: 0,
            usage: 0,
            max_mempool: 300_000_000,
            mempool_min_fee: 0.00001000,
            min_relay_tx_fee: 0.00001000,
        })
    }
}
