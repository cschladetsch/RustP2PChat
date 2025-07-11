use clap::{Parser, Subcommand};
use rust_p2p_chat::{config::Config, P2PChat};
use std::io;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "rust-p2p-chat")]
#[command(about = "A true peer-to-peer chat application", long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short = 'c', long)]
    connect: Option<String>,

    #[arg(short, long)]
    debug: bool,

    #[arg(long)]
    no_encryption: bool,

    #[arg(short, long)]
    nickname: Option<String>,

    #[arg(short, long)]
    gui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    // Simple GUI check - no Windows-specific behavior
    if cli.gui {
        println!("Launching GUI...");
        return rust_p2p_chat::gui::run_gui().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("GUI error: {}", e))
        });
    }

    // Initialize logging
    let filter = EnvFilter::new(if cli.debug { "debug" } else { "info" });
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    println!("P2P Chat Application");
    println!("====================");

    // Load config
    let mut config = Config::default();
    if let Some(nick) = cli.nickname {
        config.nickname = Some(nick);
    }
    config.enable_encryption = !cli.no_encryption;

    // Create chat instance
    let mut chat = P2PChat::new(config).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to create chat: {}", e))
    })?;

    // Start chat
    println!("Starting chat on port {} ...", cli.port);
    if let Some(peer) = &cli.connect {
        println!("Connecting to {}", peer);
    } else {
        println!("Waiting for connections...");
    }

    chat.start(cli.port, cli.connect).await.map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Chat error: {}", e))
    })?;

    Ok(())
}