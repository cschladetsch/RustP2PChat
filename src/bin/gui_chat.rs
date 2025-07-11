use rust_p2p_chat::gui::run_gui;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("rust_p2p_chat=info"));

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_level(true),
        )
        .with(filter)
        .init();

    println!("Starting Rust P2P Chat GUI...");

    // Run the GUI
    run_gui()?;

    Ok(())
}
