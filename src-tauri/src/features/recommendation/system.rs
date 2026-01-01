use super::model::{Article, ArticleCategory, UserPersona};
use super::service::{
    calculate_relevance_score, fetch_feed, recommend_with_gemini, update_user_persona,
};
use crate::repositories::article::ArticleRepository;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

pub struct RecommendationState {
    pub repository: Arc<dyn ArticleRepository + Send + Sync>,
    pub persona: Mutex<UserPersona>,
}

impl RecommendationState {
    pub fn new(repository: Arc<dyn ArticleRepository + Send + Sync>) -> Self {
        Self {
            repository,
            persona: Mutex::new(UserPersona::default()),
        }
    }

    pub fn save_persona(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let persona = self.persona.lock().unwrap();
        let persona_path = app_dir.join("user_persona.json");
        let _ = fs::write(
            persona_path,
            serde_json::to_string(&*persona).unwrap_or_default(),
        );
    }

    pub fn load_persona(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        let persona_path = app_dir.join("user_persona.json");
        if persona_path.exists() {
            if let Ok(content) = fs::read_to_string(persona_path) {
                if let Ok(saved) = serde_json::from_str::<UserPersona>(&content) {
                    *self.persona.lock().unwrap() = saved;
                }
            }
        }
    }
}

#[tauri::command]
pub async fn fetch_articles(
    state: State<'_, RecommendationState>,
    _app: tauri::AppHandle,
) -> Result<usize, String> {
    let feeds = vec![
        // Rust
        ("https://blog.rust-lang.org/feed.xml", ArticleCategory::Rust),
        (
            "https://this-week-in-rust.org/rss.xml",
            ArticleCategory::Rust,
        ),
        // Android / Kotlin
        (
            "https://feeds.feedburner.com/blogspot/hsDu",
            ArticleCategory::Android,
        ),
        ("https://androidweekly.net/rss", ArticleCategory::Android),
        // Tauri
        ("https://tauri.app/blog/rss.xml", ArticleCategory::Tauri),
        // Web / TypeScript
        (
            "https://devblogs.microsoft.com/typescript/feed/",
            ArticleCategory::TypeScript,
        ),
        ("https://css-tricks.com/feed/", ArticleCategory::Web),
        (
            "https://www.smashingmagazine.com/feed/",
            ArticleCategory::Web,
        ),
        ("https://web.dev/feed.xml", ArticleCategory::Web),
        ("https://fettblog.eu/feed.xml", ArticleCategory::TypeScript),
        (
            "https://levelup.gitconnected.com/feed",
            ArticleCategory::Web,
        ),
        (
            "https://2ality.com/feeds/posts.xml",
            ArticleCategory::TypeScript,
        ),
        // React
        ("https://react.dev/feed.xml", ArticleCategory::React),
        ("https://overreacted.io/rss.xml", ArticleCategory::React),
        ("https://tkdodo.eu/blog/rss.xml", ArticleCategory::React),
        (
            "https://kentcdodds.com/blog/rss.xml",
            ArticleCategory::React,
        ),
        (
            "https://www.joshwcomeau.com/rss.xml",
            ArticleCategory::React,
        ),
        ("https://robinwieruch.de/index.xml", ArticleCategory::React),
        ("https://ui.dev/blog/rss", ArticleCategory::React),
        (
            "https://www.developerway.com/rss.xml",
            ArticleCategory::React,
        ),
        // AI
        ("https://openai.com/blog/rss.xml", ArticleCategory::AI),
        ("https://blogs.microsoft.com/ai/feed/", ArticleCategory::AI),
        // General / Tech
        ("https://news.ycombinator.com/rss", ArticleCategory::General),
        ("https://dev.to/feed", ArticleCategory::General),
    ];

    let mut all_fetched = Vec::new();

    for (url, category) in feeds {
        if let Ok(fetched) = fetch_feed(url, category).await {
            all_fetched.extend(fetched);
        }
    }

    // Delegate persistence to the Repository
    state.repository.create_bulk(all_fetched)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
) -> Result<Vec<Article>, String> {
    // 1. Fetch from Repository
    let articles = state.repository.get_all()?;

    // 2. Calculate Scores & Sort
    let mut scored_articles: Vec<(i32, Article)> = articles
        .into_iter()
        .map(|a| (calculate_relevance_score(&a), a))
        .filter(|(score, _)| *score > -10)
        .collect();

    // Sort by Score DESC, then Date DESC
    scored_articles.sort_by(|(score_a, article_a), (score_b, article_b)| {
        score_b
            .cmp(score_a)
            .then_with(|| article_b.published_at.cmp(&article_a.published_at))
    });

    let top_candidates: Vec<Article> = scored_articles.into_iter().map(|(_, a)| a).collect();

    // 3. Rule-based: Top 3 (Highest Scored + Newest)
    let top_3: Vec<Article> = top_candidates.iter().take(3).cloned().collect();
    let remaining: Vec<Article> = top_candidates.iter().skip(3).cloned().collect();

    // 4. AI-based: Next 4 from remaining
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();

    let ai_picks = if !api_key.is_empty() && !remaining.is_empty() {
        let candidates_for_ai: Vec<Article> = remaining.into_iter().take(20).collect();
        let persona = state.persona.lock().unwrap().clone();

        // This await is now safe because we are not holding any DB locks/connections
        recommend_with_gemini(candidates_for_ai, &persona, api_key).await
    } else {
        remaining.into_iter().take(4).collect()
    };

    // 5. Combine
    let mut result = top_3;
    result.extend(ai_picks);

    Ok(result)
}

#[tauri::command]
pub async fn submit_feedback(
    id: String,
    helpful: bool,
    reason: String,
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let timestamp = chrono::Local::now().to_rfc3339();

    // 1. Update via Repository
    state.repository.update_feedback(&id, helpful, &reason, &timestamp)?;

    // 2. Fetch all feedback via Repository
    let all_feedback = state.repository.get_feedback()?;

    // 3. AI Persona Update
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if !api_key.is_empty() {
        let current_persona = state.persona.lock().unwrap().clone();

        // Async call safe here
        if let Ok(new_persona) =
            update_user_persona(&all_feedback, &current_persona, &api_key).await
        {
            *state.persona.lock().unwrap() = new_persona;
            state.save_persona(&app);
        }
    }

    Ok(())
}
