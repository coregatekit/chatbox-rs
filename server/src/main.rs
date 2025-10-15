use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server is listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    println!("New WebSocket connection established");
    
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg.expect("Failed to read message");

        if msg.is_text() {
            println!("Received: {}", msg);

            write.send(Message::Text(format!("Echo: {}", msg))).await.unwrap();
        }

    }
}
