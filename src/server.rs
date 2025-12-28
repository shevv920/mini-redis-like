use crate::command::Response;
use crate::executor::execute;
use crate::protocol::parse;
use crate::storage::Storage;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
pub async fn serve(addr: &str, storage: Arc<dyn Storage>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let cleanup_storage = storage.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            cleanup_storage.cleanup_expired();
        }
    });

    loop {
        let (stream, _) = listener.accept().await?;
        let storage = storage.clone();

        tokio::spawn(async move {
            handle_client(stream, storage).await;
        });
    }
}

async fn handle_client(mut stream: TcpStream, storage: Arc<dyn Storage>) {
    let mut buf = [0u8; 1024];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break, // client closed
            Ok(n) => {
                let response = match parse(&buf[..n]) {
                    Ok(cmd) => execute(cmd, storage.as_ref()),
                    Err(_) => Response::Error("-ERR".to_string()),
                };
                if stream.write_all(&response.to_bytes()).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
