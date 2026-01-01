use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct UserPersona {
    pub description: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ArticleCategory {
    Rust,
    Tauri,
    React,
    TypeScript,
    Android,
    Kotlin,
    Web,
    AI,
    General,
}

impl fmt::Display for ArticleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feedback {
    pub is_helpful: bool,
    pub reason: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub url: String,
    pub category: ArticleCategory,
    pub published_at: String,
    pub feedback: Option<Feedback>,
    pub image_url: Option<String>,
    pub author: Option<String>,
}
