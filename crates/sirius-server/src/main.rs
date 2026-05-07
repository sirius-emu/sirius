//! Main entry point for the Sirius Emulator.

use rand::seq::IndexedRandom;
use sirius_config::Config;
use sirius_network::{ConnectionManager, Listener, spawn_cleanup_task};
use std::net::SocketAddr;
use tokio::sync::{mpsc, watch};
use tracing::info;

fn print_sirius_banner() {
    let version = env!("CARGO_PKG_VERSION");

    let quotes = [
        "Morningstar said we'd rock in 2026. The Dogstar is ready.",
        "The brightest star in the universe.",
        "Borrow checked approved this emulator.",
        "Safe, concurrent and ready to host.",
        "Thanks for the roadmap, Morningstar.",
        "A new star for a new era.",
        "Rocking in 2026, just like they promised.",
        "Shining brighter than the Morningstar.",
        "Who needs a CMS when the emulator is this beautiful?",
        "Parsing packets faster than you can say bobba.",
    ];

    let mut rng = rand::rng();
    let selected_quote = quotes.choose(&mut rng).unwrap();

    let ascii_art = r#"
        ▄████████  ▄█     ▄████████  ▄█  ███    █▄     ▄████████
       ███    ███ ███    ███    ███ ███  ███    ███   ███    ███
       ███    █▀  ███▌   ███    ███ ███▌ ███    ███   ███    █▀
       ███        ███▌  ▄███▄▄▄▄██▀ ███▌ ███    ███   ███
     ▀███████████ ███▌ ▀▀███▀▀▀▀▀   ███▌ ███    ███ ▀███████████
              ███ ███  ▀███████████ ███  ███    ███          ███
        ▄█    ███ ███    ███    ███ ███  ███    ███    ▄█    ███
      ▄████████▀  █▀     ███    ███ █▀   ████████▀   ▄████████▀
"#;

    let banner = format!("{}\n      v{} | {}", ascii_art, version, selected_quote);

    let start_color = (200.0, 230.0, 255.0);
    let end_color = (0.0, 80.0, 255.0);

    let lines: Vec<&str> = banner.lines().collect();
    let num_lines = lines.len();

    let mut colored_banner = String::new();

    for (i, line) in lines.iter().enumerate() {
        let ratio = if num_lines > 1 {
            i as f32 / (num_lines - 1) as f32
        } else {
            0.0
        };

        let r = (start_color.0 + ratio * (end_color.0 - start_color.0)) as u8;
        let g = (start_color.1 + ratio * (end_color.1 - start_color.1)) as u8;
        let b = (start_color.2 + ratio * (end_color.2 - start_color.2)) as u8;

        colored_banner.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m\n", r, g, b, line));
    }

    println!("{}", colored_banner);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("development")?;

    sirius_tracing::init(sirius_tracing::TracingConfig::default())?;
    info!("Starting Sirius Emulator");

    let manager = ConnectionManager::new(config.network.rate_limit_per_ip as usize * 100);
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
    let listener = Listener::bind(addr, &config.network, manager, close_tx).await?;

    print_sirius_banner();

    listener
        .run(shutdown_rx, |mut connection| async move {
            info!(id = %connection.id, peer = %connection.peer_addr, "accepted new connection");

            tokio::spawn(async move {
                while let Some(packet) = connection.inbound_rx.recv().await {
                    info!(
                         id = %connection.id,
                         header_id = packet.id(),
                         len = packet.body.len(),
                         "received packet"
                    );
                }
                info!(id = %connection.id, "connection handler finished");
            });
        })
        .await;

    info!("Sirius shut down successfully");
    Ok(())
}
