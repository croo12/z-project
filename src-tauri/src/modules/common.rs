use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ArticleCategory {
    React,
    Rust,
    Android,
    Tauri,
    TypeScript,
    General,
    AI,
    Web,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feedback {
    pub is_helpful: bool,
    pub reason: String,
    pub created_at: String,
}
