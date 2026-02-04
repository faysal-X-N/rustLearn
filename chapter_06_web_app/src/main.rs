// 06_web_app: Web 全栈实战
// 目标: 构建一个基于 Axum 的 RESTful API 后端，并提供静态文件服务

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

// --- 数据模型 ---
// Derive 宏自动实现序列化(Serialize)和反序列化(Deserialize)
// Clone 用于在内存中复制数据
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    username: String,
    content: String,
}

// --- 应用状态 ---
// 我们需要在线程间共享数据，所以使用 Arc (原子引用计数)
// 我们需要修改数据，所以使用 Mutex (互斥锁)
struct AppState {
    messages: Mutex<Vec<Message>>,
}

#[tokio::main]
async fn main() {
    println!("=== 第六章: Web 全栈实战 ===");

    // 初始化应用状态
    // 这里简单起见，数据存储在内存中，重启服务器后数据会丢失
    let shared_state = Arc::new(AppState {
        messages: Mutex::new(vec![
            Message {
                username: String::from("System"),
                content: String::from("欢迎来到 Rust 留言板！"),
            }
        ]),
    });

    // 动态检测静态文件目录
    // 兼容从 workspace 根目录运行和从子目录运行两种情况
    let mut static_path = "static";
    if !std::path::Path::new(static_path).exists() {
        if std::path::Path::new("chapter_06_web_app/static").exists() {
            static_path = "chapter_06_web_app/static";
        } else {
             eprintln!("⚠️ 警告: 未找到 static 目录！网页可能无法加载。请确保在项目根目录或 chapter_06_web_app 目录下运行。");
        }
    }
    println!("📂 静态文件服务目录: {}", static_path);

    // 构建路由
    let app = Router::new()
        // API 路由
        .route("/messages", get(get_messages).post(create_message))
        // 静态文件服务 (前端页面)
        .nest_service("/", ServeDir::new(static_path))
        // 注入共享状态
        .with_state(shared_state);

    // 运行服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("服务器正在运行: http://{}", addr);
    
    // Axum 0.7 写法
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- 处理函数 ---

// GET /messages
// State<Arc<AppState>>: 提取共享状态
async fn get_messages(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Message>> {
    // 获取锁并克隆数据
    // 使用 expect 代替 unwrap，提供错误上下文
    let messages = state.messages.lock().expect("Shared state lock poisoned");
    Json(messages.clone())
}

// POST /messages
// Json(payload): 提取请求体中的 JSON 数据
async fn create_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Message>,
) -> (StatusCode, Json<Message>) {
    // 1. 输入验证
    if payload.username.is_empty() || payload.username.len() > 50 {
        // 这里简单返回 400，实际项目中可能需要更详细的错误信息结构
        // 为了演示简单，我们复用 payload 结构，或者应该定义 ErrorResponse
        // 这里仅打印日志并返回空消息表示失败（简化处理）
        eprintln!("Invalid input: username length error");
        return (StatusCode::BAD_REQUEST, Json(payload));
    }
    if payload.content.is_empty() || payload.content.len() > 1000 {
        eprintln!("Invalid input: content length error");
        return (StatusCode::BAD_REQUEST, Json(payload));
    }

    // 2. 获取锁并处理错误 (更健壮的错误处理)
    // expect 会在 panic 时提供更有意义的信息
    // 在生产环境中，这里应该使用 match 处理毒化(poisoning)或使用 parking_lot::Mutex
    let mut messages = state.messages.lock().expect("Shared state lock poisoned");
    
    messages.push(payload.clone());
    
    // 返回 201 Created 和新创建的消息
    (StatusCode::CREATED, Json(payload))
}
