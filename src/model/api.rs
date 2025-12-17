use serde::{Deserialize, Serialize};

// API 请求相关结构体
#[derive(Serialize)]
pub struct ApiRequestBody {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// API 响应相关结构体
#[derive(Deserialize)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}
