mod connection;
mod protocol;
mod cmd;

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::collections::HashMap;

type Db = Arc<Mutex<HashMap<String, Vec<u8>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = connection::handle_connection(socket, db).await {
                eprintln!("connection {} error: {}", addr, e);
            }
        });
    }
}
