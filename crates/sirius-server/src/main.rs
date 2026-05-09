//! Main entry point for the Sirius Emulator.

mod banner;
mod context;

use sirius_config::Config;
use sirius_database::Database;
use sirius_network::{ConnectionManager, Listener, spawn_cleanup_task};
use sirius_permissions::PermissionsManager;
use sirius_repository::Repository;
use sirius_session::{SessionManager, spawn_session};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, watch};
use tracing::info;

use crate::context::ServerContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let env_name = std::env::var("SIRIUS_ENV").unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            "development".to_string()
        } else {
            "production".to_string()
        }
    });

    let config = match Config::load(&env_name) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };
    let shared_config = Arc::new(config);

    if let Err(e) = sirius_tracing::init(&shared_config.tracing) {
        eprintln!("Tracing initialization error: {}", e);
        std::process::exit(1);
    }

    info!("Starting Sirius");

    let db = Database::connect(&shared_config.database).await?;
    let pool = db.pool().clone();

    info!(
        size = db.stats().size,
        idle = db.stats().idle,
        "database connected"
    );

    let repository = Repository::new(&db);

    let context = ServerContext {
        sessions: SessionManager::new(),
        repository,
    };

    let manager = ConnectionManager::new(
        shared_config.network.rate_limit_per_ip as usize * 100,
    );
    let (close_tx, close_rx) = mpsc::channel(1024);
    spawn_cleanup_task(manager.clone(), close_rx);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("received Ctrl-C, initiating graceful shutdown");
        let _ = shutdown_tx.send(true);
    });

    let ip: std::net::IpAddr = shared_config.server.bind_address.parse()?;
    let addr = SocketAddr::new(ip, shared_config.server.port);

    let listener =
        Listener::bind(addr, &shared_config.network, manager, close_tx).await?;

    let permissions = Arc::new(PermissionsManager::load(pool.clone()).await?);

    banner::print_sirius_banner(&shared_config.server.environment);

    listener
        .run(shutdown_rx, move |connection| {
            let ctx = context.clone();
            let permissions = permissions.clone();

            async move {
                spawn_session(
                    connection,
                    ctx.sessions,
                    ctx.repository,
                    permissions,
                );
            }
        })
        .await;

    info!("Sirius shut down successfully");
    Ok(())
}
