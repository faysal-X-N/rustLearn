// 导入标准库中与 TCP 网络相关的类型：TcpListener 用于监听端口，TcpStream 表示连接，Shutdown 用于关闭连接
use std::net::{TcpListener, Shutdown};
// 导入读写所需的 Trait：Read 负责读取字节、Write 负责写入字节
use std::io::{Read, Write};

fn main() { // 程序入口函数，从这里开始执行
    // 在本地地址 127.0.0.1:8080 上绑定一个 TCP 监听器；unwrap() 遇到错误直接崩溃，方便我们快速发现问题
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 打印一行提示，说明服务器已经开始监听
    println!("监听中：127.0.0.1:8080");

    // 使用无限循环，不断等待并处理新的客户端连接
    for stream_result in listener.incoming() { // incoming() 会源源不断地产生新的连接
        // 从结果中取出真正的连接流；如果获取失败，unwrap() 会让程序崩溃（此处为了简单直接）
        let mut stream = stream_result.unwrap();

        // 一旦有客户端连进来，按照需求打印一行提示
        println!("有新客户端连接了！");

        // 准备一个 1KB 的缓冲区用于读取客户端发来的数据（足够演示用）
        let mut buffer = [0u8; 1024];

        // 从连接中读取一次数据，返回实际读取的字节数 n；如果读取失败就直接崩溃
        let n = stream.read(&mut buffer).unwrap();

        // 把读到的字节尽量转换为可读的文本（遇到非 UTF-8 字节会用替代符号，避免崩溃）
        let request_text = String::from_utf8_lossy(&buffer[..n]);

        // 把客户端发来的内容打印到终端上
        println!("收到数据：\n{}", request_text);

        // 准备最简单的 HTTP 响应：状态行 + 空行 + 正文
        // 注意：\r\n 是 HTTP 规范要求的换行，浏览器会在连接关闭时认为响应结束
        let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust!";

        // 把响应完整写回给客户端
        stream.write_all(response.as_bytes()).unwrap();

        // 主动刷新缓冲区，尽快把数据发出去
        stream.flush().unwrap();

        // 断开连接（简单起见，不支持长连接）
        stream.shutdown(Shutdown::Both).unwrap();
        // 循环继续，等待下一个客户端连接
    }
    // main 函数结束，程序退出（实际上因为 for 循环是无限的，除非出错或被中断）
}