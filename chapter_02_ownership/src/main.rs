// 02_ownership: 所有权机制
// 目标: 理解所有权、借用和生命周期 (基础)

fn main() {
    println!("=== 第二章: 所有权 (Ownership) ===");

    // --- 1. 所有权规则 (Ownership Rules) ---
    // - Rust 中的每一个值都有一个被称为其 所有者 (owner) 的变量。
    // - 值在任一时刻有且只有一个所有者。
    // - 当所有者（变量）离开作用域，这个值将被丢弃。

    {
        let s = String::from("hello"); // s 进入作用域
        println!("Inside scope: {}", s);
    } // s 离开作用域，String 内存被释放

    // --- 2. 移动 (Move) ---
    let s1 = String::from("hello");
    let s2 = s1; 
    // s1 的所有权移动到了 s2。s1 不再有效。
    // println!("{}, world!", s1); // 这行代码会报错: value borrowed here after move

    println!("Move 演示: s2 = {}", s2);

    // --- 3. 克隆 (Clone) ---
    // 如果我们需要深度复制 String 中堆上的数据，而不仅仅是栈上的指针，可以使用 clone
    let s3 = s2.clone();
    println!("Clone 演示: s2 = {}, s3 = {}", s2, s3);

    // --- 4. 拷贝 (Copy) ---
    // 像整数这样的栈上数据，默认实现 Copy trait，赋值时会自动拷贝，旧变量依然有效
    let x = 5;
    let y = x;
    println!("Copy 演示: x = {}, y = {}", x, y);

    // --- 5. 引用与借用 (References and Borrowing) ---
    // 引用允许你使用值但不获取其所有权
    let s4 = String::from("hello");
    let len = calculate_length(&s4); // 传递引用，s4 依然有效
    println!("Borrow 演示: '{}' 的长度是 {}", s4, len);

    // 可变引用
    let mut s5 = String::from("hello");
    change(&mut s5); // 传递可变引用
    println!("Mutable Borrow 演示: {}", s5);

    // --- 6. 引用规则 ---
    // - 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // - 引用必须总是有效的。

    // let r1 = &mut s5;
    // let r2 = &mut s5; // 报错: cannot borrow `s5` as mutable more than once at a time
    
    // --- 7. 切片 (Slice) ---
    // 切片是引用集合中一段连续的元素序列，而不是引用整个集合
    let s6 = String::from("hello world");
    let hello = &s6[0..5]; // &str 类型
    let world = &s6[6..11];
    println!("Slice 演示: hello='{}', world='{}'", hello, world);
}

// 接收一个字符串切片 &str
fn calculate_length(s: &str) -> usize {
    s.len()
} // s 离开作用域，因为它只是引用，所以不会丢弃它指向的数据

// 接收一个 String 的可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
