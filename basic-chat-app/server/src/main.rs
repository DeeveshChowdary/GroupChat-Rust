use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(100); // for broadcasting to all clients

    println!("🚀 WebSocket server running at ws://127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut sender, mut receiver) = ws_stream.split();

            // Sending messages to the client
            let send_task = tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    if sender.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            });

            // Receiving messages from the client
            while let Some(Ok(msg)) = receiver.next().await {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    let _ = tx.send(text);
                }
            }

            send_task.abort(); // stop sending if client disconnected
        });
    }
}
