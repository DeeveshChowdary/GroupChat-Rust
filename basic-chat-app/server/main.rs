[package]
name = "server"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.20"
tungstenite = "0.20"
futures = "0.3"