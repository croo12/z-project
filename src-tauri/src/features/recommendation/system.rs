use super::model::{Article, ArticleCategory, Feedback, UserPersona};
use super::service::{
    calculate_relevance_score, fetch_feed, recommend_with_gemini, update_user_persona,
};
use crate::db::DbPool;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

pub struct RecommendationState {
    pub pool: DbPool,
    pub persona: Mutex<UserPersona>,
}

impl RecommendationState {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
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

    // Refactored to separate DB logic from async calls to solve Send issues and allow easier testing
    pub fn get_articles_from_db(&self) -> Result<Vec<Article>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;

        let mut stmt = conn.prepare("SELECT id, title, summary, url, category, published_at, feedback_helpful, feedback_reason, feedback_at FROM articles")
            .map_err(|e| e.to_string())?;

        let articles_iter = stmt
            .query_map([], |row| {
                let cat_str: String = row.get(4)?;
                let category = match cat_str.as_str() {
                    "Rust" => ArticleCategory::Rust,
                    "Android" => ArticleCategory::Android,
                    "Tauri" => ArticleCategory::Tauri,
                    "TypeScript" => ArticleCategory::TypeScript,
                    "Web" => ArticleCategory::Web,
                    "React" => ArticleCategory::React,
                    "AI" => ArticleCategory::AI,
                    "General" => ArticleCategory::General,
                    _ => ArticleCategory::General, // Fallback
                };

                let feedback_helpful: Option<bool> = row.get(6).ok();
                let feedback_reason: Option<String> = row.get(7).ok();
                let feedback_at: Option<String> = row.get(8).ok();

                let feedback = if let (Some(h), Some(r), Some(t)) =
                    (feedback_helpful, feedback_reason, feedback_at)
                {
                    Some(Feedback {
                        is_helpful: h,
                        reason: r,
                        created_at: t,
                    })
                } else {
                    None
                };

                Ok(Article {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    summary: row.get(2)?,
                    url: row.get(3)?,
                    category,
                    published_at: row.get(5)?,
                    image_url: None,
                    author: None,
                    feedback,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut articles = Vec::new();
        for a in articles_iter {
            articles.push(a.map_err(|e| e.to_string())?);
        }
        Ok(articles)
    }

    pub fn get_feedback_from_db(&self) -> Result<Vec<Feedback>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT feedback_helpful, feedback_reason, feedback_at FROM articles WHERE feedback_helpful IS NOT NULL")
             .map_err(|e| e.to_string())?;

        let feedback_iter = stmt
            .query_map([], |row| {
                Ok(Feedback {
                    is_helpful: row.get(0)?,
                    reason: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;

        Ok(feedback_iter.filter_map(|f| f.ok()).collect())
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

    let mut new_count = 0;
    let client = reqwest::Client::new();

    let mut handles = Vec::new();
    for (url, category) in feeds {
        let url = url.to_string();
        let category = category.clone();
        let client = client.clone();
        handles.push(tauri::async_runtime::spawn(async move {
            fetch_feed(&client, &url, category).await
        }));
    }

    let mut all_fetched_articles = Vec::new();
    for handle in handles {
        if let Ok(Ok(fetched)) = handle.await {
            all_fetched_articles.extend(fetched);
        }
    }

    let conn = state.pool.get().map_err(|e| e.to_string())?;

    for item in all_fetched_articles {
        let count: u32 = conn.execute(
            "INSERT OR IGNORE INTO articles (id, title, summary, url, category, published_at, feedback_helpful, feedback_reason, feedback_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                item.id,
                item.title,
                item.summary,
                item.url,
                item.category.to_string(),
                item.published_at,
                item.feedback.as_ref().map(|f| f.is_helpful),
                item.feedback.as_ref().map(|f| f.reason.clone()),
                item.feedback.as_ref().map(|f| f.created_at.clone())
            ],
        ).map_err(|e| e.to_string())? as u32;

        new_count += count as usize;
    }

    Ok(new_count)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
) -> Result<Vec<Article>, String> {
    // DB Access must happen synchronously and drop connection before async AI calls
    let articles = state.get_articles_from_db()?;

    // 1. Calculate Scores & Sort
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

    // 2. Rule-based: Top 3 (Highest Scored + Newest)
    let top_3: Vec<Article> = top_candidates.iter().take(3).cloned().collect();
    let remaining: Vec<Article> = top_candidates.iter().skip(3).cloned().collect();

    // 3. AI-based: Next 4 from remaining
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();

    let ai_picks = if !api_key.is_empty() && !remaining.is_empty() {
        let candidates_for_ai: Vec<Article> = remaining.into_iter().take(20).collect();
        let persona = state.persona.lock().unwrap().clone();

        // This await is now safe because we are not holding any DB locks/connections
        recommend_with_gemini(candidates_for_ai, &persona, api_key).await
    } else {
        remaining.into_iter().take(4).collect()
    };

    // 4. Combine
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
    // 1. Update DB (scoped)
    {
        let conn = state.pool.get().map_err(|e| e.to_string())?;
        let timestamp = chrono::Local::now().to_rfc3339();
        conn.execute(
            "UPDATE articles SET feedback_helpful = ?1, feedback_reason = ?2, feedback_at = ?3 WHERE id = ?4",
            rusqlite::params![helpful, reason, timestamp, id],
        ).map_err(|e| e.to_string())?;
    } // conn dropped

    // 2. Fetch all feedback (scoped)
    let all_feedback = state.get_feedback_from_db()?;

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
