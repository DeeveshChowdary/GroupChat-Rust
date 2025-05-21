use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Message};

type Tx = broadcast::Sender<String>;
type SharedRooms = Arc<Mutex<HashMap<String, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let rooms: SharedRooms = Arc::new(Mutex::new(HashMap::new()));

    println!("🚀 WebSocket server with chat room support running at ws://127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let rooms = Arc::clone(&rooms);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut sender, mut receiver) = ws_stream.split();

            let mut current_room = "global".to_string();

            // Join default room
            let (tx, mut rx) = {
                let mut map = rooms.lock().unwrap();
                map.entry(current_room.clone())
                    .or_insert_with(|| broadcast::channel(100).0)
                    .clone()
            };

            // Forward broadcast messages to this client
            let mut send_task = tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    if sender
                        .send(Message::Text(format!("{}", msg)))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            });

            // Receive messages from client
            while let Some(Ok(msg)) = receiver.next().await {
                if let Message::Text(text) = msg {
                    if text.starts_with("/join ") {
                        let new_room = text[6..].trim().to_string();

                        let (new_tx, new_rx) = {
                            let mut map = rooms.lock().unwrap();
                            map.entry(new_room.clone())
                                .or_insert_with(|| broadcast::channel(100).0)
                                .clone()
                        };

                        current_room = new_room;
                        rx = new_rx;
                        println!("User {} joined room '{}'", addr, current_room);
                        let _ = new_tx.send(format!("👋 {} joined the room!", addr));
                    } else {
                        let _ = tx.send(format!("[{}] {}", addr, text));
                    }
                }
            }

            send_task.abort();
        });
    }
}
