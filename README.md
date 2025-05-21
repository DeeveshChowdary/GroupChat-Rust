Context for curious programmers:

In the Rust programming language, Cargo.toml is the manifest file for Rust's package manager, Cargo. This file contains metadata such as the project name, version, and dependencies (called "crates" in Rust). It's essentially a configuration file that defines how Cargo should build and manage your Rust project. 

🚀 How to Run

# Step 1: Create the project
mkdir chat-ws && cd chat-ws
cargo new server --bin
cargo new client --bin

# Step 2: Run the server
cargo run -p server

# Step 3: In another terminal, run the client (or multiple clients)
cargo run -p client

Now you can chat between multiple terminals!


## 🚀 Features

- **Real-time chat using WebSockets**
- **Multiple concurrent clients**
- **NEW in v1.1:** Chat room support
  - Clients can join rooms using `/join <room-name>`
  - Messages are only visible within the room
  - Default room: `global`

## 📌 Version History

### v1.1 (May 2025)
- Added support for chat rooms
- Clients can switch rooms using `/join <room-name>`
- Each room maintains its own broadcast channel
