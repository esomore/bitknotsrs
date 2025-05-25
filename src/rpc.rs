use jsonrpc_core::{IoHandler, Params, Result as RpcResult, Value};
use jsonrpc_http_server::{ServerBuilder, Server};
use serde_json::json;
use std::net::SocketAddr;
use tracing::{info, error};

use crate::config::{Config, RpcConfig};
use crate::error::{RpcError, NodeResult};

pub struct RpcServer {
    _server: Server,
}

pub async fn start_server(config: &Config) -> NodeResult<RpcServer> {
    let mut io = IoHandler::new();

    // Register RPC methods
    register_blockchain_methods(&mut io);
    register_network_methods(&mut io);
    register_transaction_methods(&mut io);
    register_utility_methods(&mut io);

    let addr: SocketAddr = format!("{}:{}", config.rpc.host, config.rpc.port)
        .parse()
        .map_err(|e| RpcError::Internal(format!("Invalid RPC address: {}", e)))?;

    let server = ServerBuilder::new(io)
        .start_http(&addr)
        .map_err(|e| RpcError::Internal(format!("Failed to start RPC server: {}", e)))?;

    info!("RPC server started on {}", addr);

    Ok(RpcServer { _server: server })
}

fn register_blockchain_methods(io: &mut IoHandler) {
    // getblockchaininfo
    io.add_method("getblockchaininfo", |_params: Params| async {
        Ok(json!({
            "chain": "regtest",
            "blocks": 0,
            "headers": 0,
            "bestblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
            "difficulty": 1.0,
            "mediantime": 0,
            "verificationprogress": 1.0,
            "initialblockdownload": false,
            "chainwork": "0000000000000000000000000000000000000000000000000000000000000000",
            "size_on_disk": 0,
            "pruned": false
        }))
    });

    // getbestblockhash
    io.add_method("getbestblockhash", |_params: Params| async {
        Ok(json!("0000000000000000000000000000000000000000000000000000000000000000"))
    });

    // getblock
    io.add_method("getblock", |params: Params| async {
        let params = params.parse::<(String, Option<u8>)>()
            .map_err(|_| jsonrpc_core::Error::invalid_params("Invalid parameters"))?;

        let _block_hash = params.0;
        let _verbosity = params.1.unwrap_or(1);

        // TODO: Get actual block data
        Ok(json!({
            "hash": "0000000000000000000000000000000000000000000000000000000000000000",
            "confirmations": 1,
            "size": 285,
            "strippedsize": 285,
            "weight": 1140,
            "height": 0,
            "version": 1,
            "versionHex": "00000001",
            "merkleroot": "0000000000000000000000000000000000000000000000000000000000000000",
            "tx": [],
            "time": 0,
            "mediantime": 0,
            "nonce": 0,
            "bits": "207fffff",
            "difficulty": 1.0,
            "chainwork": "0000000000000000000000000000000000000000000000000000000000000002",
            "nTx": 0,
            "previousblockhash": null,
            "nextblockhash": null
        }))
    });

    // getblockcount
    io.add_method("getblockcount", |_params: Params| async {
        Ok(json!(0))
    });

    // getblockhash
    io.add_method("getblockhash", |params: Params| async {
        let params = params.parse::<(u64,)>()
            .map_err(|_| jsonrpc_core::Error::invalid_params("Invalid parameters"))?;

        let _height = params.0;

        // TODO: Get actual block hash for height
        Ok(json!("0000000000000000000000000000000000000000000000000000000000000000"))
    });
}

fn register_network_methods(io: &mut IoHandler) {
    // getnetworkinfo
    io.add_method("getnetworkinfo", |_params: Params| async {
        Ok(json!({
            "version": 250000,
            "subversion": "/BitKnotsRS:0.1.0/",
            "protocolversion": 70016,
            "localservices": "0000000000000409",
            "localservicesnames": ["NETWORK", "WITNESS", "NETWORK_LIMITED"],
            "localrelay": true,
            "timeoffset": 0,
            "connections": 0,
            "connections_in": 0,
            "connections_out": 0,
            "networkactive": true,
            "networks": [],
            "relayfee": 0.00001000,
            "incrementalfee": 0.00001000,
            "localaddresses": [],
            "warnings": ""
        }))
    });

    // getpeerinfo
    io.add_method("getpeerinfo", |_params: Params| async {
        // TODO: Get actual peer information
        Ok(json!([]))
    });

    // getconnectioncount
    io.add_method("getconnectioncount", |_params: Params| async {
        Ok(json!(0))
    });
}

fn register_transaction_methods(io: &mut IoHandler) {
    // getrawtransaction
    io.add_method("getrawtransaction", |params: Params| async {
        let params = params.parse::<(String, Option<bool>)>()
            .map_err(|_| jsonrpc_core::Error::invalid_params("Invalid parameters"))?;

        let _txid = params.0;
        let verbose = params.1.unwrap_or(false);

        if verbose {
            // TODO: Get actual transaction data
            Ok(json!({
                "txid": "0000000000000000000000000000000000000000000000000000000000000000",
                "hash": "0000000000000000000000000000000000000000000000000000000000000000",
                "version": 1,
                "size": 0,
                "vsize": 0,
                "weight": 0,
                "locktime": 0,
                "vin": [],
                "vout": [],
                "hex": "",
                "blockhash": null,
                "confirmations": 0,
                "time": 0,
                "blocktime": 0
            }))
        } else {
            // Return raw hex
            Ok(json!(""))
        }
    });

    // sendrawtransaction
    io.add_method("sendrawtransaction", |params: Params| async {
        let params = params.parse::<(String,)>()
            .map_err(|_| jsonrpc_core::Error::invalid_params("Invalid parameters"))?;

        let _hex = params.0;

        // TODO: Validate and broadcast transaction
        Ok(json!("0000000000000000000000000000000000000000000000000000000000000000"))
    });

    // getmempoolinfo
    io.add_method("getmempoolinfo", |_params: Params| async {
        Ok(json!({
            "loaded": true,
            "size": 0,
            "bytes": 0,
            "usage": 0,
            "maxmempool": 300000000,
            "mempoolminfee": 0.00001000,
            "minrelaytxfee": 0.00001000,
            "unbroadcastcount": 0
        }))
    });

    // getrawmempool
    io.add_method("getrawmempool", |params: Params| async {
        let verbose = if let Ok((verbose,)) = params.parse::<(bool,)>() {
            verbose
        } else {
            false
        };

        if verbose {
            // TODO: Get actual mempool data with details
            Ok(json!({}))
        } else {
            // TODO: Get actual mempool transaction IDs
            Ok(json!([]))
        }
    });
}

fn register_utility_methods(io: &mut IoHandler) {
    // help
    io.add_method("help", |params: Params| async {
        let command = if let Ok((cmd,)) = params.parse::<(String,)>() {
            Some(cmd)
        } else {
            None
        };

        match command.as_deref() {
            Some("getblockchaininfo") => Ok(json!("getblockchaininfo\n\nReturns an object containing various state info regarding blockchain processing.")),
            Some("getbestblockhash") => Ok(json!("getbestblockhash\n\nReturns the hash of the best (tip) block in the most-work fully-validated chain.")),
            Some("getblock") => Ok(json!("getblock \"blockhash\" ( verbosity )\n\nIf verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.")),
            Some("getblockcount") => Ok(json!("getblockcount\n\nReturns the height of the most-work fully-validated chain.")),
            Some("getblockhash") => Ok(json!("getblockhash height\n\nReturns hash of block in best-block-chain at height provided.")),
            Some("getnetworkinfo") => Ok(json!("getnetworkinfo\n\nReturns an object containing various state info regarding P2P networking.")),
            Some("getpeerinfo") => Ok(json!("getpeerinfo\n\nReturns data about each connected network node as a json array of objects.")),
            Some("getconnectioncount") => Ok(json!("getconnectioncount\n\nReturns the number of connections to other nodes.")),
            Some("getrawtransaction") => Ok(json!("getrawtransaction \"txid\" ( verbose \"blockhash\" )\n\nReturn the raw transaction data.")),
            Some("sendrawtransaction") => Ok(json!("sendrawtransaction \"hexstring\" ( maxfeerate )\n\nSubmit a raw transaction (serialized, hex-encoded) to local node and network.")),
            Some("getmempoolinfo") => Ok(json!("getmempoolinfo\n\nReturns details on the active state of the TX memory pool.")),
            Some("getrawmempool") => Ok(json!("getrawmempool ( verbose )\n\nReturns all transaction ids in memory pool as a json array of string transaction ids.")),
            None => Ok(json!(
                "Available commands:\n\
                getblockchaininfo\n\
                getbestblockhash\n\
                getblock\n\
                getblockcount\n\
                getblockhash\n\
                getnetworkinfo\n\
                getpeerinfo\n\
                getconnectioncount\n\
                getrawtransaction\n\
                sendrawtransaction\n\
                getmempoolinfo\n\
                getrawmempool\n\
                help"
            )),
            Some(_) => Ok(json!("Unknown command. Use 'help' to list available commands.")),
        }
    });

    // uptime
    io.add_method("uptime", |_params: Params| async {
        // TODO: Calculate actual uptime
        Ok(json!(0))
    });

    // getversion (non-standard but useful)
    io.add_method("getversion", |_params: Params| async {
        Ok(json!({
            "version": env!("CARGO_PKG_VERSION"),
            "name": "BitKnotsRS",
            "description": "A knots-inspired Bitcoin node implementation in Rust"
        }))
    });
}