use tokio_tungstenite::connect_async;
use futures::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    println!("Connected to chat. Type your messages below:");

    // Task to receive and print messages
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
                println!("\nReceived: {}", txt);
            }
        }
    });

    // Read from stdin and send to server
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        write.send(tokio_tungstenite::tungstenite::Message::Text(line)).await.unwrap();
    }
}
