//! Main entry point for the Sirius Emulator.

use sirius_config::Config;
use sirius_network::{ConnectionManager, Listener, spawn_cleanup_task};
use std::net::SocketAddr;
use tokio::sync::{mpsc, watch};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load configuration
    let config = Config::load("development")?;

    // 2. Initialize tracing (logging)
    sirius_tracing::init(sirius_tracing::TracingConfig::default())?;
    info!("Starting Sirius Emulator");

    // 3. Create connection manager
    let manager = ConnectionManager::new(config.network.rate_limit_per_ip as usize * 100); // Simple upper bound for max capacity for now
    let (close_tx, close_rx) = mpsc::channel(1024);
    spawn_cleanup_task(manager.clone(), close_rx);

    // 4. Setup graceful shutdown
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Received Ctrl-C, initiating graceful shutdown");
        let _ = shutdown_tx.send(true);
    });

    // 5. Bind listener
    let ip: std::net::IpAddr = config.server.bind_address.parse()?;
    let addr = SocketAddr::new(ip, config.server.port);
    let listener = Listener::bind(addr, &config.network, manager, close_tx).await?;

    // 6. Accept loop
    listener
        .run(shutdown_rx, |mut connection| async move {
            info!(id = %connection.id, peer = %connection.peer_addr, "accepted new connection");

            // STUB: Since sirius-session isn't implemented yet, we'll just read
            // packets and log them to verify multiplexing (TCP/WS) and codecs work.
            tokio::spawn(async move {
                while let Some(packet) = connection.inbound_rx.recv().await {
                    info!(
                        id = %connection.id,
                        header_id = packet.id(),
                        len = packet.body.len(),
                        "received packet"
                    );
                    
                    // We can echo it back or just ignore it.
                    // For now, let's just log it.
                }
                info!(id = %connection.id, "connection handler finished");
            });
        })
        .await;

    info!("Sirius Emulator shut down successfully");
    Ok(())
}
