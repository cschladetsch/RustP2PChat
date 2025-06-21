use std::io::{self, Write};
use rust_p2p_chat::P2PPeer;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("\x1b[1;36m╔═══════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║           \x1b[1;93m✨ True P2P Chat Application ✨\x1b[1;36m                ║\x1b[0m");
    println!("\x1b[1;36m╚═══════════════════════════════════════════════════════════╝\x1b[0m");
    println!("\x1b[90mUsage:\x1b[0m");
    println!("  \x1b[32m1.\x1b[0m Run with just a port: \x1b[93m./rust-p2p-chat 8080\x1b[0m");
    println!("  \x1b[32m2.\x1b[0m Run with port and peer address: \x1b[93m./rust-p2p-chat 8081 127.0.0.1:8080\x1b[0m");
    println!();

    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    let (listen_port, peer_address) = if args.len() < 2 {
        // Interactive mode if no arguments provided
        print!("Enter port to listen on (default 8080): ");
        io::stdout().flush()?;
        
        let mut port_input = String::new();
        io::stdin().read_line(&mut port_input)?;
        let port = port_input.trim().parse::<u16>().unwrap_or(8080);
        
        print!("Enter peer address (e.g., 127.0.0.1:8081) or press Enter to wait for connection: ");
        io::stdout().flush()?;
        
        let mut peer_input = String::new();
        io::stdin().read_line(&mut peer_input)?;
        let peer_addr = peer_input.trim();
        
        let peer_address = if peer_addr.is_empty() {
            None
        } else {
            Some(peer_addr.to_string())
        };
        
        (port, peer_address)
    } else if args.len() == 2 {
        // Just port provided
        let port = args[1].parse::<u16>().expect("Invalid port number");
        (port, None)
    } else {
        // Port and peer address provided
        let port = args[1].parse::<u16>().expect("Invalid port number");
        let peer_addr = args[2].clone();
        (port, Some(peer_addr))
    };

    // Create and start the P2P peer
    let peer = P2PPeer::new(listen_port, peer_address);
    peer.start().await?;

    Ok(())
}