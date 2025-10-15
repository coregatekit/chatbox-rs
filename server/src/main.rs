use futures::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_tungstenite::accept_async;
use tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server is listening on {}", addr);

    let (tx, _rx) = broadcast::channel::<String>(100);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let rx = tx.subscribe();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, tx, rx).await {
                eprintln!("Error in connection handler: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");
    println!("🟢 New client connected");

    let (mut write, mut read) = ws_stream.split();

    // Task for receiving messages from the client
    let tx_clone = tx.clone();
    let read_handle = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("📩 Received: {}", text);
                let _ = tx_clone.send(text);
            }
        }
    });

    // Task for sending messages to the client
    let write_handle = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if write.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = read_handle => (),
        _ = write_handle => (),
    }

    println!("🔴 Client disconnected");
    Ok(())
}
