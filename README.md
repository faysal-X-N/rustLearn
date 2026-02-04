# Rust 全栈学习路径 (Rust Full Stack Learning Path)

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

这是一个专为零基础学习者设计的 Rust 语言实战项目。通过 6 个循序渐进的章节，从基础语法到构建完整的 Web 全栈应用，帮助你快速掌握 Rust 核心概念。

## 📚 内容概览

本项目包含以下章节（详细学习路径请参考 [LEARNING_PATH.md](LEARNING_PATH.md)）：

| 章节 | 内容 | 核心概念 |
| :--- | :--- | :--- |
| **01_basics** | 基础语法 | 变量、数据类型、流程控制 |
| **02_ownership** | 核心机制 | **所有权**、借用、Slice |
| **03_structs_enums** | 数据结构 | 结构体、枚举、模式匹配 |
| **04_error_handling** | 错误处理 | Result, Option, `?` 运算符 |
| **05_concurrency** | 并发编程 | 线程、Channel、Mutex |
| **06_web_app** | **全栈实战** | Axum 后端 + HTML 前端留言板 |

## 🚀 快速开始

### 环境要求
- [Rust](https://www.rust-lang.org/tools/install) (建议最新版)

### 运行方式

1. **克隆仓库**
   ```bash
   git clone https://github.com/your-username/rust-learning-path.git
   cd rust-learning-path
   ```

2. **运行特定章节**
   使用 Cargo 在根目录即可运行任意章节：
   ```bash
   # 运行第一章：基础语法
   cargo run -p chapter_01_basics
   ```

3. **运行 Web 实战项目**
   ```bash
   cargo run -p chapter_06_web_app
   ```
   启动后访问: [http://localhost:3000](http://localhost:3000)

## 📝 学习指南

*   每个章节的源代码 (`src/main.rs`) 都包含了**详细的中文注释**，请务必仔细阅读。
*   建议按照章节顺序进行学习和实践。
*   项目已配置好 Cargo Workspace，可以直接在根目录进行构建和测试。

## 🤝 贡献

欢迎提交 Issue 或 Pull Request 来改进本项目！

## 📄 许可证

本项目采用 MIT 许可证。
