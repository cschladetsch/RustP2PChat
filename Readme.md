# Rust P2P Chat

A simple peer-to-peer chat application written in Rust using Tokio for async networking.

## Demo

![Demo(resources/Demo.png)

## Features

- Direct peer-to-peer connection (no central server)
- Real-time bidirectional text messaging
- Simple console interface
- TCP-based communication

## Usage

### Running the application

```bash
cargo run
```

### Starting a chat session

1. **As a server (waiting for connection):**
   - Run the application
   - Press Enter when prompted for peer address
   - Enter a port number (default 8080)
   - Wait for a peer to connect

2. **As a client (connecting to a peer):**
   - Run the application
   - Enter the peer's address in format `ip:port` (e.g., `127.0.0.1:8080`)
   - Connection will be established automatically

### Example usage

**Terminal 1 (Server):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 
Enter port to listen on (default 8080): 8080
Listening on: 0.0.0.0:8080
Waiting for peer to connect...
```

**Terminal 2 (Client):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 127.0.0.1:8080
Connecting to 127.0.0.1:8080...
Connected to peer!
```

### Sending messages

Once connected, type messages and press Enter to send. Messages from the peer will appear prefixed with "Peer:".

To exit, press Ctrl+C.

## Technical Details

- Uses Tokio for async I/O
- Implements concurrent reading and writing using Arc<Mutex<TcpStream>>
- Handles connection errors and peer disconnection gracefully

## Testing

The project includes comprehensive unit and integration tests.

### Running Tests

Run all tests:
```bash
cargo test
```

Run unit tests only:
```bash
cargo test --lib
```

Run integration tests only:
```bash
cargo test --test simple_integration_test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

### Test Coverage

The test suite includes:
- Unit tests for server and client creation
- TCP connection establishment tests
- Message exchange verification
- Bidirectional communication tests
- Large message handling
- Multiple sequential message tests
- Connection error handling

## Project Structure

```
rust-p2p-chat/
├── Cargo.toml           # Project dependencies
├── Readme.md            # This file
├── src/
│   ├── main.rs         # Application entry point
│   └── lib.rs          # Core chat functionality
└── tests/
    ├── integration_tests.rs      # Complex integration tests
    └── simple_integration_test.rs # Basic integration tests
```

## Testing Locally

### Testing with Two Terminals

#### Method 1: Both terminals on same machine

**Terminal 1 (Server):**
```bash
cargo run
# Press Enter when prompted for peer address
# Enter port (or press Enter for default 8080)
```

**Terminal 2 (Client):**
```bash
cargo run
# Enter: 127.0.0.1:8080
```

#### Method 2: Quick test commands

**Terminal 1:**
```bash
cargo run --release
# Press Enter, then Enter again (uses default port 8080)
```

**Terminal 2:**
```bash
cargo run --release
# Type: 127.0.0.1:8080
```

### Example Session

**Terminal 1 (Server):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 
Enter port to listen on (default 8080): 
Listening on: 0.0.0.0:8080
Waiting for peer to connect...
Peer connected from: 127.0.0.1:xxxxx
Type messages and press Enter to send (Ctrl+C to exit)
You: Hello from server!
Peer: Hi from client!
You: How are you?
```

**Terminal 2 (Client):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 127.0.0.1:8080
Connecting to 127.0.0.1:8080...
Connected to peer!
Type messages and press Enter to send (Ctrl+C to exit)
You: Hi from client!
Peer: Hello from server!
You: I'm doing great!
```

### Testing Between Different Machines

1. **Find server's IP address:**
   ```bash
   # Linux/Mac
   ip addr show | grep inet
   # or
   ifconfig
   
   # Windows
   ipconfig
   ```

2. **On server machine:**
   ```bash
   cargo run
   # Press Enter, use port 8080
   ```

3. **On client machine:**
   ```bash
   cargo run
   # Enter: SERVER_IP:8080 (e.g., 192.168.1.100:8080)
   ```

### Alternative Testing Methods

#### Using tmux or screen:

```bash
# Install tmux if not already installed
sudo apt install tmux  # Ubuntu/Debian
# or
brew install tmux      # macOS

# Start tmux session
tmux new -s chat-test

# Split window horizontally (Ctrl+B then ")
# Switch between panes with Ctrl+B then arrow keys

# In first pane: run server
cargo run
# Press Enter twice

# In second pane: run client  
cargo run
# Type: 127.0.0.1:8080
```

### Testing specific scenarios:

1. **Test connection refusal:**
   ```bash
   cargo run
   # Enter: 127.0.0.1:9999 (non-existent server)
   ```

2. **Test with custom port:**
   ```bash
   # Server
   cargo run
   # Press Enter, then type: 3000
   
   # Client
   cargo run  
   # Type: 127.0.0.1:3000
   ```

3. **Test rapid messages:**
   Once connected, type multiple messages quickly to test buffering.

### Tips:
- Use `cargo run --release` for better performance
- Messages appear prefixed with "Peer:" when received
- Press Ctrl+C to exit cleanly
- The app handles disconnection gracefully - you'll see "Peer disconnected"
