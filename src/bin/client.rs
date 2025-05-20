use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Receive message from stdin
            Ok(Some(line)) = stdin.next_line() => {
                ws_stream.send(Message::text(line)).await?;
            }

            // Get incoming messages from server
            Some(msg) = ws_stream.next() => {
                let msg = msg?;
                if msg.is_text() {
                    println!("Chat from {}", msg.as_text().unwrap().to_string());
                }
            }
            
            else => break,
        }
    }

    Ok(())
}