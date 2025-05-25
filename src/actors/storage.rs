use actix::prelude::*;
use tracing::{info, error};
use bitcoin::hashes::Hash;

use crate::config::Config;
use crate::storage::Storage;
use crate::error::{StorageError, StorageResult};
use super::{StoreBlock, GetBlock, AddTransaction, GetTransaction};

pub struct StorageActor {
    storage: Storage,
}

impl StorageActor {
    pub fn new(config: &Config) -> Self {
        let storage = Storage::new(&config.storage)
            .expect("Failed to initialize storage");

        info!("Storage actor initialized");

        Self { storage }
    }
}

impl Actor for StorageActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Storage actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Storage actor stopped");
    }
}

impl Handler<StoreBlock> for StorageActor {
    type Result = Result<(), StorageError>;

    fn handle(&mut self, msg: StoreBlock, _ctx: &mut Self::Context) -> Self::Result {
        let block_hash = msg.block.block_hash();
        let block_data = bitcoin::consensus::serialize(&msg.block);

        self.storage.store_block(&block_hash.to_byte_array(), &block_data)?;

        info!("Stored block: {}", block_hash);
        Ok(())
    }
}

impl Handler<GetBlock> for StorageActor {
    type Result = Result<Option<bitcoin::Block>, StorageError>;

    fn handle(&mut self, msg: GetBlock, _ctx: &mut Self::Context) -> Self::Result {
        match self.storage.get_block(&msg.hash.to_byte_array())? {
            Some(block_data) => {
                match bitcoin::consensus::deserialize(&block_data) {
                    Ok(block) => Ok(Some(block)),
                    Err(e) => {
                        error!("Failed to deserialize block {}: {}", msg.hash, e);
                        Err(StorageError::Serialization(e.to_string()))
                    }
                }
            }
            None => Ok(None),
        }
    }
}

impl Handler<AddTransaction> for StorageActor {
    type Result = Result<(), StorageError>;

    fn handle(&mut self, msg: AddTransaction, _ctx: &mut Self::Context) -> Self::Result {
        let txid = msg.tx.txid();
        let tx_data = bitcoin::consensus::serialize(&msg.tx);

        self.storage.store_transaction(&txid.to_byte_array(), &tx_data)?;

        info!("Stored transaction: {}", txid);
        Ok(())
    }
}

impl Handler<GetTransaction> for StorageActor {
    type Result = Result<Option<bitcoin::Transaction>, StorageError>;

    fn handle(&mut self, msg: GetTransaction, _ctx: &mut Self::Context) -> Self::Result {
        match self.storage.get_transaction(&msg.txid.to_byte_array())? {
            Some(tx_data) => {
                match bitcoin::consensus::deserialize(&tx_data) {
                    Ok(tx) => Ok(Some(tx)),
                    Err(e) => {
                        error!("Failed to deserialize transaction {}: {}", msg.txid, e);
                        Err(StorageError::Serialization(e.to_string()))
                    }
                }
            }
            None => Ok(None),
        }
    }
}