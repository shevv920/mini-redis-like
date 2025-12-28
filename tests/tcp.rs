#[tokio::test]
async fn tcp_set_get() {
    use untitled::server::serve;
    use untitled::storage::memory::MemoryStorage;
    use std::sync::Arc;
    use tokio::net::TcpStream;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::time::{sleep, Duration};
    use tokio::task;

    let storage = Arc::new(MemoryStorage::new());
    let addr = "127.0.0.1:6380";

    task::spawn({
        let storage = storage.clone();
        async move {
            serve(addr, storage).await.unwrap();
        }
    });

    sleep(Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(addr).await.unwrap();
    stream.write_all(b"SET a 1\n").await.unwrap();

    let mut buf = [0u8; 32];
    let n = stream.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"+OK\n"); // match your Response serialization
}
