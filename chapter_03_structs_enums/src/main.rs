// 03_structs_enums: 结构体与枚举
// 目标: 掌握自定义数据类型和模式匹配

// 允许死代码警告 (因为这是教学示例，某些字段可能未被使用)
#![allow(dead_code)]

// --- 定义结构体 ---
// 派生 Debug trait 以便使用 {:?} 打印
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// --- 结构体方法 ---
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法: 第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数: 不以 self 为参数，通常用于构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// --- 定义枚举 ---
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // 可以直接包含数据
    V6(String),
}

// --- 消息枚举示例 ---
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 包含匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called: {:?}", self);
    }
}

fn main() {
    println!("=== 第三章: 结构体与枚举 ===");

    // 1. 使用结构体
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {:?}", user1);

    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 其他字段使用 user1 的值
    };
    println!("User2: {:?}", user2);

    // 2. 方法与关联函数
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rect1 面积: {}", rect1.area());

    let sq = Rectangle::square(20);
    println!("Square: {:?}", sq);

    // 3. 枚举的使用
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("IPs: {:?}, {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // 4. Option 枚举
    // Rust 没有 Null，而是使用 Option<T> { Some(T), None }
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("Option: {:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // 5. match 控制流
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // 6. if let 简单控制流
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

#[derive(Debug)] // 允许打印
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 绑定值的模式
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
