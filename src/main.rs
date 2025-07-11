use clap::{Parser, Subcommand};
use rust_p2p_chat::{config::Config, P2PChat};
use std::io;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "rust-p2p-chat")]
#[command(about = "A true peer-to-peer chat application", long_about = None)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Peer address to connect to (e.g., 192.168.1.100:8080)
    #[arg(short = 'c', long)]
    connect: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Disable encryption
    #[arg(long)]
    no_encryption: bool,

    /// Set nickname
    #[arg(short, long)]
    nickname: Option<String>,

    /// Launch GUI interface
    #[arg(short, long)]
    gui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and save default configuration
    Config,
}

#[cfg(windows)]
#[windows_subsystem = "windows"] // This prevents console window on Windows when using GUI

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    // On Windows, default to GUI mode if no arguments provided
    #[cfg(windows)]
    let use_gui = cli.gui || (std::env::args().len() == 1);
    
    #[cfg(not(windows))]
    let use_gui = cli.gui;

    // Initialize tracing/logging
    let log_level = if cli.debug { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("rust_p2p_chat={}", log_level)));

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_level(true),
        )
        .with(filter)
        .init();

    info!("Starting Rust P2P Chat v{}", env!("CARGO_PKG_VERSION"));

    // Handle GUI mode
    if use_gui {
        info!("Launching GUI interface with drag & drop support");
        return rust_p2p_chat::gui::run_gui().map_err(|e| {
            error!("GUI error: {}", e);
            io::Error::new(io::ErrorKind::Other, format!("GUI error: {}", e))
        });
    }

    // Handle special commands
    if let Some(command) = cli.command {
        match command {
            Commands::Config => {
                info!("Generating default configuration");
                let config = Config::default();
                match config.save() {
                    Ok(_) => {
                        info!("Configuration saved successfully");
                        println!("Configuration saved successfully!");
                        println!("Edit the config file to customize settings.");
                    }
                    Err(e) => {
                        error!("Failed to save configuration: {}", e);
                        eprintln!("Failed to save configuration: {}", e);
                    }
                }
                return Ok(());
            }
        }
    }

    // Load or create configuration
    let mut config = match Config::load() {
        Ok(cfg) => {
            debug!("Loaded configuration from file");
            cfg
        }
        Err(e) => {
            warn!("Could not load config file ({}), using defaults", e);
            Config::default()
        }
    };

    // Override config with CLI arguments
    if cli.debug {
        debug!("Debug logging enabled via CLI");
        config.log_level = "debug".to_string();
    }
    if cli.no_encryption {
        warn!("Encryption disabled via CLI - messages will be unencrypted!");
        config.enable_encryption = false;
    }
    if let Some(nick) = &cli.nickname {
        info!("Setting nickname to: {}", nick);
        config.nickname = cli.nickname;
    }

    // Display banner in interactive mode
    if std::env::args().len() < 2 {
        println!("\x1b[1;36m╔═══════════════════════════════════════════════════════════╗\x1b[0m");
        println!("\x1b[1;36m║           \x1b[1;93m✨ True P2P Chat Application ✨\x1b[1;36m                ║\x1b[0m");
        println!("\x1b[1;36m╚═══════════════════════════════════════════════════════════╝\x1b[0m");
        println!("\x1b[90mUsage:\x1b[0m");
        println!(
            "  \x1b[32m1.\x1b[0m Run with just a port: \x1b[93mrust-p2p-chat --port 8080\x1b[0m"
        );
        println!("  \x1b[32m2.\x1b[0m Connect to a peer: \x1b[93mrust-p2p-chat --connect 192.168.1.100:8080\x1b[0m");
        println!("  \x1b[32m3.\x1b[0m Set nickname: \x1b[93mrust-p2p-chat --nickname Alice\x1b[0m");
        println!("  \x1b[32m4.\x1b[0m Type \x1b[93m/help\x1b[0m for available commands");
        println!();
    }

    // Create and start the chat
    info!(
        "Initializing P2P chat with port {} and encryption {}",
        cli.port,
        if config.enable_encryption {
            "enabled"
        } else {
            "disabled"
        }
    );

    let mut chat = P2PChat::new(config).map_err(|e| {
        error!("Failed to create chat: {}", e);
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create chat: {}", e),
        )
    })?;

    if let Some(ref peer_addr) = cli.connect {
        info!("Attempting to connect to peer at: {}", peer_addr);
    } else {
        info!("Starting in listen mode on port: {}", cli.port);
    }

    chat.start(cli.port, cli.connect).await.map_err(|e| {
        error!("Chat session ended with error: {}", e);
        io::Error::new(io::ErrorKind::Other, format!("Chat error: {}", e))
    })?;

    info!("Chat session ended normally");

    Ok(())
}
