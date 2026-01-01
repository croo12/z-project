use super::model::{Article, ArticleCategory, Feedback, UserPersona};
use super::service::{
    calculate_relevance_score, fetch_feed, recommend_with_gemini, update_user_persona,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

pub struct RecommendationState {
    pub articles: Mutex<Vec<Article>>,
    pub persona: Mutex<UserPersona>,
}

impl Default for RecommendationState {
    fn default() -> Self {
        Self {
            articles: Mutex::new(Vec::new()),
            persona: Mutex::new(UserPersona::default()),
        }
    }
}

impl RecommendationState {
    pub fn save_articles(&self, app_handle: &tauri::AppHandle) {
        let articles = self.articles.lock().unwrap();
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let path = app_dir.join("articles_v2.json");
        let _ = fs::write(path, serde_json::to_string(&*articles).unwrap_or_default());

        // Save Persona
        let persona = self.persona.lock().unwrap();
        let persona_path = app_dir.join("user_persona.json");
        let _ = fs::write(
            persona_path,
            serde_json::to_string(&*persona).unwrap_or_default(),
        );
    }

    pub fn load_articles(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));

        let path = app_dir.join("articles_v2.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(saved) = serde_json::from_str::<Vec<Article>>(&content) {
                    *self.articles.lock().unwrap() = saved;
                }
            }
        }

        // Load Persona
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
    app: tauri::AppHandle,
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
        // General / Tech (Filtered heavily by score)
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

    let mut articles = state.articles.lock().unwrap();
    for item in all_fetched_articles {
        if !articles.iter().any(|a| a.id == item.id) {
            articles.push(item);
            new_count += 1;
        }
    }

    state.save_articles(&app);
    Ok(new_count)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
) -> Result<Vec<Article>, String> {
    let articles = state.articles.lock().unwrap().clone();

    // 1. Calculate Scores & Sort
    let mut scored_articles: Vec<(i32, Article)> = articles
        .into_iter()
        .map(|a| (calculate_relevance_score(&a), a))
        .filter(|(score, _)| *score > -10) // Filter out very negative articles (e.g. pure noise)
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
        // Use top 20 remaining as candidates for AI mostly to save tokens,
        // relying on our scoring to have put the good stuff in these top 20.
        let candidates_for_ai: Vec<Article> = remaining.into_iter().take(20).collect();

        let persona = state.persona.lock().unwrap().clone();
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
    let all_feedback: Vec<Feedback>;
    {
        let mut articles = state.articles.lock().unwrap();
        if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
            article.feedback = Some(Feedback {
                is_helpful: helpful,
                reason,
                created_at: chrono::Local::now().to_rfc3339(),
            });
            // Clone feedback for async processing
            all_feedback = articles.iter().filter_map(|a| a.feedback.clone()).collect();
        } else {
            return Err("Article not found".to_string());
        }
    } // MutexGuard dropped here, safe to await later

    state.save_articles(&app);

    // AI Persona Update
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if !api_key.is_empty() {
        let current_persona = state.persona.lock().unwrap().clone();
        // Call AI service
        if let Ok(new_persona) =
            update_user_persona(&all_feedback, &current_persona, &api_key).await
        {
            // Update State
            *state.persona.lock().unwrap() = new_persona;
            // Save Persona (via save_articles helper or directly, here helper saves both)
            state.save_articles(&app);
        }
    }

    Ok(())
}
