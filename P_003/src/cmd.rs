use crate::protocol::RespValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn execute(args: Vec<Vec<u8>>, db: Arc<Mutex<HashMap<String, Vec<u8>>>>) -> RespValue {
    if args.is_empty() {
        return RespValue::Error("ERR empty command".into());
    }
    let cmd_upper = String::from_utf8_lossy(&args[0]).to_ascii_uppercase();
    match cmd_upper.as_str() {
        "PING" => {
            RespValue::SimpleString("PONG".into())
        }
        "SET" => {
            if args.len() != 3 {
                return RespValue::Error("ERR wrong number of arguments for 'SET'".into());
            }
            let key = match String::from_utf8(args[1].clone()) {
                Ok(s) => s,
                Err(_) => return RespValue::Error("ERR key must be UTF-8".into()),
            };
            let value = args[2].clone();
            let mut guard = db.lock().await;
            guard.insert(key, value);
            RespValue::SimpleString("OK".into())
        }
        "GET" => {
            if args.len() != 2 {
                return RespValue::Error("ERR wrong number of arguments for 'GET'".into());
            }
            let key = match String::from_utf8(args[1].clone()) {
                Ok(s) => s,
                Err(_) => return RespValue::Error("ERR key must be UTF-8".into()),
            };
            let guard = db.lock().await;
            match guard.get(&key) {
                Some(v) => RespValue::BulkString(Some(v.clone())),
                None => RespValue::BulkString(None),
            }
        }
        _ => RespValue::Error("ERR unknown command".into()),
    }
}
