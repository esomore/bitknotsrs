use actix::prelude::*;
use tracing::{info, error};

use crate::config::Config;
use crate::error::NetworkError;
use super::{NewPeer, DisconnectPeer, GetPeers, PeerInfo, BroadcastTransaction, BroadcastBlock};

pub struct NetworkActor {
    _storage_actor: Addr<super::storage::StorageActor>,
}

impl NetworkActor {
    pub fn new(config: &Config, storage_actor: Addr<super::storage::StorageActor>) -> Self {
        info!("Network actor initialized");
        Self {
            _storage_actor: storage_actor,
        }
    }
}

impl Actor for NetworkActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Network actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Network actor stopped");
    }
}

impl Handler<NewPeer> for NetworkActor {
    type Result = Result<(), NetworkError>;

    fn handle(&mut self, msg: NewPeer, _ctx: &mut Self::Context) -> Self::Result {
        info!("New peer connected: {} from {}", msg.peer_id, msg.address);
        // TODO: Implement peer connection logic
        Ok(())
    }
}

impl Handler<DisconnectPeer> for NetworkActor {
    type Result = Result<(), NetworkError>;

    fn handle(&mut self, msg: DisconnectPeer, _ctx: &mut Self::Context) -> Self::Result {
        info!("Peer disconnected: {} ({})", msg.peer_id, msg.reason);
        // TODO: Implement peer disconnection logic
        Ok(())
    }
}

impl Handler<GetPeers> for NetworkActor {
    type Result = Result<Vec<PeerInfo>, NetworkError>;

    fn handle(&mut self, _msg: GetPeers, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: Return actual peer list
        Ok(vec![])
    }
}

impl Handler<BroadcastTransaction> for NetworkActor {
    type Result = Result<(), NetworkError>;

    fn handle(&mut self, msg: BroadcastTransaction, _ctx: &mut Self::Context) -> Self::Result {
        info!("Broadcasting transaction: {}", msg.tx.txid());
        // TODO: Implement transaction broadcasting
        Ok(())
    }
}

impl Handler<BroadcastBlock> for NetworkActor {
    type Result = Result<(), NetworkError>;

    fn handle(&mut self, msg: BroadcastBlock, _ctx: &mut Self::Context) -> Self::Result {
        info!("Broadcasting block: {}", msg.block.block_hash());
        // TODO: Implement block broadcasting
        Ok(())
    }
}