use thiserror::Error;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Error)]
pub enum RespError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("协议解析错误: {0}")]
    Parse(String),
    #[error("连接已关闭")]
    Eof,
}

#[derive(Debug, Clone)]
pub enum RespValue {
    SimpleString(String),
    BulkString(Option<Vec<u8>>),
    Array(Vec<RespValue>),
    Error(String),
    Integer(i64),
}

// 读取一行以 CRLF 结尾的字节，去掉结尾的 \r\n 并返回行内容
async fn read_crlf_line<R: AsyncBufRead + Unpin>(r: &mut R) -> Result<Vec<u8>, RespError> {
    let mut buf = Vec::with_capacity(64);
    let n = r.read_until(b'\n', &mut buf).await?;
    if n == 0 {
        return Err(RespError::Eof);
    }
    if buf.len() < 2 || buf[buf.len() - 2] != b'\r' || buf[buf.len() - 1] != b'\n' {
        return Err(RespError::Parse("行未以 CRLF 结尾".into()));
    }
    buf.truncate(buf.len() - 2);
    Ok(buf)
}

// 解析十进制数字（可能带负号）的 ASCII 表示
fn parse_decimal_i64(bytes: &[u8]) -> Result<i64, RespError> {
    let s = std::str::from_utf8(bytes).map_err(|_| RespError::Parse("数字包含非 UTF-8 字节".into()))?;
    s.parse::<i64>().map_err(|_| RespError::Parse("数字解析失败".into()))
}

// 解析 RESP 数组（仅支持由批量字符串组成的数组），返回 Vec<Vec<u8>> 参数列表
//
// 状态机逻辑（仅数组 + 批量字符串子集）：
// 1. 首字节必须是 '*'，随后读取一行得到数组元素个数 N
// 2. 对于每个元素：
//    2.1 读取首字节，必须是 '$'
//    2.2 读取一行得到长度 L；若 L = -1 则表示 nil（此处视为错误，因为命令参数不允许为 nil）
//    2.3 读取 L 个字节作为内容，再读 CRLF
// 3. 返回收集到的 N 个参数
pub async fn read_array_of_bulk_strings<R: AsyncBufRead + Unpin>(r: &mut R) -> Result<Vec<Vec<u8>>, RespError> {
    let mut prefix = [0u8; 1];
    r.read_exact(&mut prefix).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF")
        } else {
            e
        }
    })?;
    if prefix[0] != b'*' {
        return Err(RespError::Parse("期望数组标识 '*'".into()));
    }
    let line = read_crlf_line(r).await?;
    let n = parse_decimal_i64(&line)?;
    if n < 0 {
        return Err(RespError::Parse("数组元素个数不能为负".into()));
    }
    let mut args = Vec::with_capacity(n as usize);
    for _ in 0..n {
        // 每个元素必须是批量字符串
        let mut p = [0u8; 1];
        r.read_exact(&mut p).await?;
        if p[0] != b'$' {
            return Err(RespError::Parse("命令参数必须为批量字符串".into()));
        }
        let line = read_crlf_line(r).await?;
        let len = parse_decimal_i64(&line)?;
        if len == -1 {
            return Err(RespError::Parse("命令参数不允许为 nil".into()));
        }
        if len < -1 {
            return Err(RespError::Parse("非法的批量字符串长度".into()));
        }
        let mut buf = vec![0u8; len as usize];
        r.read_exact(&mut buf).await?;
        let mut crlf = [0u8; 2];
        r.read_exact(&mut crlf).await?;
        if crlf != [b'\r', b'\n'] {
            return Err(RespError::Parse("批量字符串未正确以 CRLF 结尾".into()));
        }
        args.push(buf);
    }
    Ok(args)
}

pub async fn write_simple_string<W: AsyncWrite + Unpin>(w: &mut W, s: &str) -> Result<(), RespError> {
    w.write_all(b"+").await?;
    w.write_all(s.as_bytes()).await?;
    w.write_all(b"\r\n").await?;
    Ok(())
}

pub async fn write_error<W: AsyncWrite + Unpin>(w: &mut W, s: &str) -> Result<(), RespError> {
    w.write_all(b"-").await?;
    w.write_all(s.as_bytes()).await?;
    w.write_all(b"\r\n").await?;
    Ok(())
}

pub async fn write_bulk_string<W: AsyncWrite + Unpin>(w: &mut W, data: &[u8]) -> Result<(), RespError> {
    let header = format!("${}\r\n", data.len());
    w.write_all(header.as_bytes()).await?;
    w.write_all(data).await?;
    w.write_all(b"\r\n").await?;
    Ok(())
}

pub async fn write_nil_bulk<W: AsyncWrite + Unpin>(w: &mut W) -> Result<(), RespError> {
    w.write_all(b"$-1\r\n").await?;
    Ok(())
}
