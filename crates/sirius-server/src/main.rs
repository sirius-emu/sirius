//! Main entry point for the Sirius Emulator.

mod banner;

use sirius_config::Config;
use sirius_database::Database;
use sirius_network::{ConnectionManager, Listener, spawn_cleanup_task};
use sirius_session::{SessionManager, spawn_session};
use std::net::SocketAddr;
use tokio::sync::{mpsc, watch};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_name = std::env::var("SIRIUS_ENV").unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            "development".to_string()
        } else {
            "production".to_string()
        }
    });

    let config = Config::load(&env_name)?;

    sirius_tracing::init(&config.tracing)?;
    info!("Starting Sirius");

    let db = Database::connect(&config.database).await?;

    info!(
        size = db.stats().size,
        idle = db.stats().idle,
        "database connected"
    );

    let manager =
        ConnectionManager::new(config.network.rate_limit_per_ip as usize * 100);
    let (close_tx, close_rx) = mpsc::channel(1024);
    spawn_cleanup_task(manager.clone(), close_rx);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("received Ctrl-C, initiating graceful shutdown");
        let _ = shutdown_tx.send(true);
    });

    let ip: std::net::IpAddr = config.server.bind_address.parse()?;
    let addr = SocketAddr::new(ip, config.server.port);
    let listener =
        Listener::bind(addr, &config.network, manager, close_tx).await?;

    banner::print_sirius_banner(config.server.environment);

    let session_manager = SessionManager::new();

    listener
        .run(shutdown_rx, move |connection| {
            let session_manager = session_manager.clone();
            async move {
                spawn_session(connection, session_manager);
            }
        })
        .await;

    info!("Sirius shut down successfully");
    Ok(())
}
