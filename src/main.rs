use actix::prelude::*;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use tracing::{info, warn, error};

mod config;
mod logging;
mod metrics;
mod events;
mod api;
mod rpc;
mod storage;
mod actors;
mod error;

use config::Config;
use error::NodeError;

#[derive(Parser)]
#[command(name = "bitknotsrs")]
#[command(about = "BitKnotsRS - A knots-inspired Bitcoin node implementation")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(long)]
    network: Option<String>,

    #[arg(long)]
    datadir: Option<String>,

    #[arg(long)]
    generate_config: bool,
}

#[actix_web::main]
async fn main() -> Result<(), NodeError> {
    let cli = Cli::parse();

    // Generate default config if requested
    if cli.generate_config {
        let config = Config::default_regtest();
        let toml_str = toml::to_string_pretty(&config)?;
        std::fs::write(&cli.config, toml_str)?;
        println!("Generated default config at: {}", cli.config);
        return Ok(());
    }

    // Load configuration
    let mut config = Config::load(&cli.config)?;

    // Override config with CLI args
    if let Some(network) = cli.network {
        config.network = network.parse()?;
    }
    if let Some(datadir) = cli.datadir {
        config.datadir = datadir.into();
    }

    // Initialize logging
    logging::init(&config.logging)?;

    info!("Starting BitKnotsRS node");
    info!("Network: {:?}", config.network);
    info!("Data directory: {:?}", config.datadir);

    // Initialize metrics
    let _metrics_handle = if config.metrics.enabled {
        Some(metrics::init(&config.metrics).await?)
    } else {
        None
    };

    // Initialize event publishers
    let event_manager = events::EventManager::new(&config).await?;

    // Start actor system
    let system = System::new();

    // Initialize storage
    let storage_actor = actors::storage::StorageActor::new(&config).start();

    // Initialize other core actors
    let _network_actor = actors::network::NetworkActor::new(&config, storage_actor.clone()).start();
    let _mempool_actor = actors::mempool::MempoolActor::new(&config, storage_actor.clone()).start();
    let _chain_actor = actors::chain::ChainActor::new(&config, storage_actor.clone()).start();

    // Start HTTP API server
    let config_clone = config.clone();
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(event_manager.clone()))
            .route("/health", web::get().to(api::health))
            .service(web::scope("/api/v1").configure(api::configure))
    })
    .bind(format!("{}:{}", config.api.host, config.api.port))?;

    info!("API server starting on {}:{}", config.api.host, config.api.port);

    // Start RPC server
    let _rpc_server = if config.rpc.enabled {
        Some(rpc::start_server(&config).await?)
    } else {
        None
    };

    // Run the server
    api_server.run().await?;

    Ok(())
}
