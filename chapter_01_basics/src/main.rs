// 01_basics: Rust 基础语法
// 目标: 了解变量、数据类型、函数和控制流

fn main() {
    println!("=== 第一章: Rust 基础语法 ===");

    // 1. 变量与可变性
    // Rust 中变量默认是不可变的 (immutable)
    let x = 5;
    println!("x 的值是: {}", x);
    // x = 6; // 这行代码会报错，因为 x 不可变

    // 使用 mut 关键字声明可变变量
    let mut y = 10;
    println!("y 的初始值: {}", y);
    y = 20;
    println!("y 修改后的值: {}", y);

    // 常量 (Constants)
    // 常量总是不可变的，并且必须注明类型
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS: {}", MAX_POINTS);

    // 2. 数据类型
    // 标量类型: 整数, 浮点数, 布尔值, 字符
    let integer: i32 = -42;
    let float: f64 = std::f64::consts::PI;
    let boolean: bool = true;
    let character: char = 'R'; // 单引号用于字符，支持 Unicode

    println!("数据类型演示: 整数={}, 浮点={}, 布尔={}, 字符={}", integer, float, boolean, character);

    // 复合类型: 元组 (Tuple) 和 数组 (Array)
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (t1, t2, t3) = tuple; // 解构
    println!("元组解构: {}, {}, {}", t1, t2, t3);
    println!("元组索引访问: {}", tuple.0);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组的第一个元素: {}", array[0]);

    // 3. 函数
    let sum = add(5, 10);
    println!("函数调用结果 (5 + 10): {}", sum);

    // 4. 控制流
    let number = 7;

    // if-else
    if number < 5 {
        println!("条件判断: number 小于 5");
    } else {
        println!("条件判断: number 大于或等于 5");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 }; // if 也是表达式
    println!("if 表达式的结果: {}", number);

    // 循环 (Loop)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以返回值
        }
    };
    println!("loop 循环的结果: {}", result);

    // while 循环
    let mut n = 3;
    while n != 0 {
        println!("while 倒计时: {}", n);
        n -= 1;
    }

    // for 循环 (最常用)
    print!("for 循环遍历数组: ");
    for element in array.iter() {
        print!("{} ", element);
    }
    println!();

    print!("for 循环范围 (1..4): ");
    for number in 1..4 { // 1 到 3，不包含 4
        print!("{} ", number);
    }
    println!();
}

// 函数定义
// 参数必须声明类型，返回值类型在 -> 之后声明
fn add(a: i32, b: i32) -> i32 {
    // Rust 中，代码块的最后一行如果不加分号，就是返回值 (表达式)
    // 如果加了分号，就是语句，返回单元类型 ()
    a + b 
}
