# Rust 语言全栈学习路径

欢迎来到 Rust 学习项目！本项目旨在通过一系列渐进式的实践章节，帮助零基础学习者掌握 Rust 语言的核心概念，并最终能够开发一个包含前后端交互的 Web 应用。

## 📚 学习导航

建议按照以下顺序阅读代码和运行项目：

### 第一阶段：基础语法 (chapter_01_basics)
*   **目标**: 熟悉 Rust 的基本语法、变量声明和控制流。
*   **核心概念**:
    *   `let` 与 `mut` (不可变性)
    *   基本数据类型 (整数, 浮点, 布尔, 字符)
    *   流程控制 (`if`, `loop`, `while`, `for`)
    *   函数基础
*   **实践**: 运行 `cargo run -p chapter_01_basics`

### 第二阶段：核心机制 (chapter_02_ownership)
*   **目标**: 理解 Rust 最独特且最重要的内存管理机制——所有权。
*   **核心概念**:
    *   所有权规则 (Ownership Rules)
    *   借用与引用 (Borrowing & References)
    *   切片 (Slices)
    *   生命周期简介
*   **实践**: 运行 `cargo run -p chapter_02_ownership`

### 第三阶段：结构化数据 (chapter_03_structs_enums)
*   **目标**: 学习如何定义自定义类型和处理复杂数据。
*   **核心概念**:
    *   结构体 (Structs)
    *   枚举 (Enums)
    *   模式匹配 (`match` & `if let`)
    *   方法 (Methods)
*   **实践**: 运行 `cargo run -p chapter_03_structs_enums`

### 第四阶段：错误处理与泛型 (chapter_04_error_handling)
*   **目标**: 掌握 Rust 健壮的错误处理机制。
*   **核心概念**:
    *   `Result<T, E>` 与 `Option<T>`
    *   `?` 运算符
    *   泛型 (Generics)
    *   Trait (特征) 基础
*   **实践**: 运行 `cargo run -p chapter_04_error_handling`

### 第五阶段：并发编程 (chapter_05_concurrency)
*   **目标**: 利用 Rust 的“无畏并发”特性编写多线程程序。
*   **核心概念**:
    *   线程 (Threads)
    *   消息传递 (Channels)
    *   共享状态 (Mutex, Arc)
*   **实践**: 运行 `cargo run -p chapter_05_concurrency`

### 第六阶段：Web 全栈实战 (chapter_06_web_app)
*   **目标**: 综合运用所学知识，构建一个前后端交互的 Web 应用。
*   **技术栈**:
    *   后端: Rust + Axum (Web 框架) + Serde (序列化)
    *   前端: HTML + JavaScript (Fetch API)
*   **功能**:
    *   提供 RESTful API
    *   静态文件服务
    *   简单的留言板功能
*   **运行**:
    1. 运行后端: `cargo run -p chapter_06_web_app`
    2. 打开浏览器访问: `http://localhost:3000`

---

## 🛠️ 如何开始

1.  确保已安装 Rust 环境 (运行 `rustc --version` 检查)。
2.  在项目根目录下，你可以使用 Cargo 运行任意章节，例如：
    ```bash
    cargo run -p chapter_01_basics
    ```
3.  **阅读源码**: 每个章节的 `src/main.rs` 中都包含了详细的**中文注释**，请务必仔细阅读。

## 💡 学习建议

*   不要急于求成，Rust 的所有权概念可能需要时间消化。
*   多动手修改代码，看看编译器会报什么错，Rust 的编译器报错信息非常友好且有教育意义。
*   尝试自己扩展每个章节的练习。

祝学习愉快！
