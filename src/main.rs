use std::io::{self, Write};
use rust_p2p_chat::{ChatServer, ChatClient};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("P2P Chat Application");
    println!("Usage:");
    println!("  1. Start as server: just press Enter when prompted for address");
    println!("  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)");
    println!();

    print!("Enter peer address (or press Enter to start as server): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let address = input.trim();

    if address.is_empty() {
        print!("Enter port to listen on (default 8080): ");
        io::stdout().flush()?;
        
        let mut port_input = String::new();
        io::stdin().read_line(&mut port_input)?;
        let port = port_input.trim().parse::<u16>().unwrap_or(8080);
        
        let server = ChatServer::new(port);
        server.start().await?;
    } else {
        let client = ChatClient::new(address.to_string());
        client.connect().await?;
    }

    Ok(())
}

