// 05_concurrency: 并发编程
// 目标: 学习线程、消息传递和共享状态

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    println!("=== 第五章: 并发编程 ===");

    // --- 1. 创建线程 (spawn) ---
    // thread::spawn 接收一个闭包，在新线程中运行代码
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("新线程: 计数 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("主线程: 计数 {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待子线程结束
    handle.join().unwrap();

    // --- 2. 线程与 move 闭包 ---
    let v = vec![1, 2, 3];
    // 使用 move 关键字强制将 v 的所有权移动到闭包中
    let handle = thread::spawn(move || {
        println!("Move 闭包演示: v = {:?}", v);
    });
    handle.join().unwrap();

    // --- 3. 消息传递 (Channels) ---
    // mpsc: multiple producer, single consumer (多生产者，单消费者)
    let (tx, rx) = mpsc::channel();

    // 创建发送者线程
    thread::spawn(move || {
        let val = String::from("hi");
        println!("发送端: 发送 '{}'", val);
        tx.send(val).unwrap(); // 发送数据
        // val 在这里已经不能用了，所有权被转移到了通道中
    });

    // 主线程接收
    let received = rx.recv().unwrap(); // 阻塞直到收到消息
    println!("接收端: 收到 '{}'", received);

    // --- 4. 共享状态 (Mutex & Arc) ---
    // Mutex: 互斥锁，一次只允许一个线程访问数据
    // Arc: 原子引用计数 (Atomic Reference Counting)，允许多个线程拥有同一个锁的所有权

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 克隆 Arc 增加引用计数
        let handle = thread::spawn(move || {
            // 获取锁
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // 锁在这里离开作用域被自动释放
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Mutex 演示: 最终计数结果 = {}", *counter.lock().unwrap());
}
