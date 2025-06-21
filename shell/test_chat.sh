#!/bin/bash

echo "Starting P2P Chat Test"
echo "====================="
echo ""
echo "1. This script will open two terminals"
echo "2. First terminal will be the server"
echo "3. Second terminal will be the client"
echo ""
echo "Press Enter to continue..."
read

# Start server in background
echo "Starting server on port 8080..."
gnome-terminal -- bash -c "cd $(pwd) && echo -e '\n8080' | cargo run; read -p 'Press Enter to close...'" &

# Wait a bit for server to start
sleep 3

# Start client
echo "Starting client connecting to 127.0.0.1:8080..."
gnome-terminal -- bash -c "cd $(pwd) && echo '127.0.0.1:8080' | cargo run; read -p 'Press Enter to close...'" &

echo ""
echo "Two terminals should now be open for testing!"
echo "Type messages in either terminal and press Enter to send."
echo "Press Ctrl+C in either terminal to exit."