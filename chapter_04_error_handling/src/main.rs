// 04_error_handling: 错误处理
// 目标: 学习 Rust 中的可恢复错误 (Result) 和不可恢复错误 (panic!)

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== 第四章: 错误处理 ===");

    // --- 1. 不可恢复错误 panic! ---
    // panic!("Crash and burn"); // 取消注释这行代码会直接导致程序崩溃

    // --- 2. 可恢复错误 Result<T, E> ---
    let f = File::open("hello.txt"); // 文件可能不存在

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            println!("无法打开文件: {:?}", error);
            // 这里我们演示了错误处理，实际项目中可能会根据 ErrorKind 进一步处理
            // 为避免后续代码报错，我们在这里 panic，或者给一个默认值，或者提前 return
            // return; 
            // 既然是演示，我们手动创建一个模拟文件对象，或者继续执行
            // 为了让代码跑通，我们不使用这个文件句柄了
            return;
        }
    };

    // --- 3. unwrap 和 expect ---
    // unwrap: 如果 Ok 返回值，如果 Err 则 panic
    // let f = File::open("hello.txt").unwrap(); 

    // expect: 类似于 unwrap，但可以指定 panic 的错误信息
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // --- 4. 传播错误 ---
    // 见 read_username_from_file 函数
    match read_username_from_file() {
        Ok(s) => println!("读取到的用户名: {}", s),
        Err(e) => println!("读取用户名失败: {}", e),
    }
}

// 演示错误传播和 ? 运算符
// 这个函数尝试打开文件并读取内容
// 如果成功返回 Ok(String)，失败返回 Err(io::Error)
fn read_username_from_file() -> Result<String, io::Error> {
    // 方式 1: 使用 match 显式处理
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 方式 2: 使用 ? 运算符 (语法糖)
    // 如果 Result 是 Ok，? 表达式的值就是 Ok 中的值
    // 如果 Result 是 Err，? 会直接从当前函数返回这个 Err
    
    // 注意：这里我们尝试读取 Cargo.toml，因为它肯定存在
    let mut s = String::new();
    // 链式调用
    File::open("Cargo.toml")?.read_to_string(&mut s)?; 
    
    // 读取前 20 个字符作为示例
    if s.len() > 20 {
        Ok(s[0..20].to_string())
    } else {
        Ok(s)
    }
}
