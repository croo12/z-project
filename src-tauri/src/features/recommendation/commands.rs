use super::model::{Article, ArticleCategory, Feedback, UserPersona, UserPreferences};
use super::repository::RecommendationRepository;
use super::service::{
    calculate_relevance_score, fetch_feed, recommend_with_gemini, update_user_persona,
};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

pub struct RecommendationState {
    pub repo: Arc<dyn RecommendationRepository + Send + Sync>,
    pub persona: Mutex<UserPersona>,
}

impl RecommendationState {
    pub fn new(repo: Arc<dyn RecommendationRepository + Send + Sync>) -> Self {
        Self {
            repo,
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

    pub fn save_preferences(&self, prefs: UserPreferences, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let prefs_path = app_dir.join("user_preferences.json");
        let _ = fs::write(
            prefs_path,
            serde_json::to_string(&prefs).unwrap_or_default(),
        );
    }

    pub fn load_preferences(&self, app_handle: &tauri::AppHandle) -> UserPreferences {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        let prefs_path = app_dir.join("user_preferences.json");
        if prefs_path.exists() {
            if let Ok(content) = fs::read_to_string(prefs_path) {
                if let Ok(saved) = serde_json::from_str::<UserPreferences>(&content) {
                    return saved;
                }
            }
        }
        UserPreferences::default()
    }
}

fn get_feeds() -> Vec<(&'static str, ArticleCategory)> {
    vec![
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
    ]
}

#[tauri::command]
pub async fn fetch_articles(state: State<'_, RecommendationState>) -> Result<usize, String> {
    let feeds = get_feeds();

    let mut new_count = 0;
    let mut all_fetched = Vec::new();

    for (url, category) in feeds {
        if let Ok(fetched) = fetch_feed(url, category).await {
            all_fetched.extend(fetched);
        }
    }

    // Deduplication & Merge Logic
    // We cannot access DB async easily if we don't have a pool, but here we use the repo
    // Note: State must be Sync/Send. repo is Arc<dyn RecommendationRepository + Send + Sync>

    // We can't do complex transaction logic inside the repo easily if we want to keep it generic,
    // but here we can iterate. Ideally, the repo should handle "upsert batch".
    // For now, let's do it one by one or improve repo API later.
    // To match previous logic, we need to check existence and merge tags.

    for mut item in all_fetched {
        let existing_tags = state.repo.get_existing_tags(&item.url)?;

        if let Some(mut current_tags) = existing_tags {
             for new_tag in item.tags {
                if !current_tags.contains(&new_tag) {
                    current_tags.push(new_tag);
                }
            }
            item.tags = current_tags;
        } else {
            new_count += 1;
        }

        // We need to preserve existing feedback/author/image if we are just updating tags?
        // The previous SQL used ON CONFLICT DO UPDATE SET tags, published_at, image_url, author.
        // It did NOT update feedback.
        // My repo implementation of `save_article` does upsert.
        // However, we need to make sure we don't overwrite feedback with None if it exists.
        // But `item` comes from RSS, so it has None feedback.
        // The SQL in previous implementation preserved other fields implicitly or explicitly?
        // Previous SQL: ON CONFLICT(url) DO UPDATE SET tags = ?5, published_at = ?6, image_url = ?7, author = ?8
        // It did NOT touch feedback columns in the update clause.
        // My repo `save_article` does: ON CONFLICT(url) DO UPDATE SET tags = ?5, published_at = ?6, image_url = ?7, author = ?8
        // So it matches the behavior. Feedback columns are untouched on update.

        state.repo.save_article(&item)?;
    }

    Ok(new_count)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<Vec<Article>, String> {
    let articles = state.repo.get_articles()?;

    // 1. Calculate Scores & Sort
    let prefs = state.load_preferences(&app);
    let mut scored_articles: Vec<(i32, Article)> = articles
        .into_iter()
        .map(|a| (calculate_relevance_score(&a, &prefs.interested_tags), a))
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

        recommend_with_gemini(candidates_for_ai, &persona, &prefs.interested_tags, api_key).await
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
    let timestamp = chrono::Local::now().to_rfc3339();
    state.repo.update_feedback(&id, helpful, &reason, &timestamp)?;

    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if !api_key.is_empty() {
        let count = state.repo.get_feedback_count().unwrap_or(0);

        if count > 0 && count % 3 == 0 {
            println!("Triggering Persona Update (Feedback Count: {})", count);
            let all_feedback = state.repo.get_feedback()?;
            let current_persona = state.persona.lock().unwrap().clone();

            if let Ok(new_persona) =
                update_user_persona(&all_feedback, &current_persona, &api_key).await
            {
                *state.persona.lock().unwrap() = new_persona;
                state.save_persona(&app);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn save_user_interests(
    categories: Vec<ArticleCategory>,
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut prefs = state.load_preferences(&app);
    prefs.interested_tags = categories;
    state.save_preferences(prefs, &app);
    Ok(())
}

#[tauri::command]
pub async fn get_user_interests(
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<Vec<ArticleCategory>, String> {
    let prefs = state.load_preferences(&app);
    Ok(prefs.interested_tags)
}
