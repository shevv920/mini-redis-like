mod command;
mod protocol;
mod server;
mod storage;
mod executor;

use std::sync::Arc;
use crate::storage::memory::MemoryStorage;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let storage = Arc::new(MemoryStorage::new());
    server::serve("127.0.0.1:6379", storage).await
}