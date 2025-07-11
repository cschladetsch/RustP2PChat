#!/usr/bin/env python3
import subprocess
import time

print("Rust P2P Chat Launcher")
print("======================\n")

mode = input("Start as (1) Alice [listener] or (2) Bob [connector]? ")

if mode == "1":
    print("\nStarting Alice on port 8080...")
    subprocess.Popen(["target\\release\\rust-p2p-chat.exe", "--port", "8080", "--nickname", "Alice"])
    print("Alice started. She's listening on port 8080.")
    
elif mode == "2":
    print("\nStarting Bob connecting to localhost:8080...")
    subprocess.Popen(["target\\release\\rust-p2p-chat.exe", "--port", "8081", "--connect", "localhost:8080", "--nickname", "Bob"])
    print("Bob started. He's connecting to Alice.")
    
else:
    print("Invalid choice!")

input("\nPress Enter to exit...")