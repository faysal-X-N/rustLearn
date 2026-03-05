use crate::protocol::{self, RespError};
use crate::cmd;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn handle_connection(stream: TcpStream, db: Arc<Mutex<HashMap<String, Vec<u8>>>>) -> Result<(), Box<dyn std::error::Error>> {
    let (r, w) = stream.into_split();
    let mut reader = BufReader::new(r);
    let mut writer = BufWriter::new(w);

    loop {
        let args = match protocol::read_array_of_bulk_strings(&mut reader).await {
            Ok(a) => a,
            Err(RespError::Eof) => break,
            Err(e) => {
                let _ = protocol::write_error(&mut writer, &format!("ERR {}", e)).await;
                let _ = writer.flush().await;
                break;
            }
        };

        let resp = cmd::execute(args, db.clone()).await;
        match resp {
            crate::protocol::RespValue::SimpleString(s) => protocol::write_simple_string(&mut writer, &s).await?,
            crate::protocol::RespValue::BulkString(Some(b)) => protocol::write_bulk_string(&mut writer, &b).await?,
            crate::protocol::RespValue::BulkString(None) => protocol::write_nil_bulk(&mut writer).await?,
            crate::protocol::RespValue::Error(e) => protocol::write_error(&mut writer, &e).await?,
            crate::protocol::RespValue::Integer(n) => {
                let s = format!(":{}\r\n", n);
                writer.write_all(s.as_bytes()).await?;
            }
            crate::protocol::RespValue::Array(_) => {
                protocol::write_error(&mut writer, "ERR nested arrays not supported in response").await?;
            }
        }
        writer.flush().await?;
    }

    Ok(())
}
