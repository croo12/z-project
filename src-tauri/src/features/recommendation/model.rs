use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct UserPersona {
    pub description: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    // Transient field for runtime scoring, not saved/serialized often? 
    // Or we keep it simple and just calculate on fly.
    // Making it optional would be fine, but for now we won't serialize it.
}
