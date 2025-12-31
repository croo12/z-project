use crate::modules::common::{ArticleCategory, Feedback};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Serialize, Deserialize, Clone)]
pub struct NewsItem {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub url: String,
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

pub struct NewsState {
    pub articles: Mutex<Vec<Article>>,
}

impl Default for NewsState {
    fn default() -> Self {
        Self {
            articles: Mutex::new(Vec::new()),
        }
    }
}

impl NewsState {
    pub fn save_articles(&self, app_handle: &tauri::AppHandle) {
        let articles = self.articles.lock().unwrap();
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let path = app_dir.join("articles.json");
        let _ = fs::write(path, serde_json::to_string(&*articles).unwrap_or_default());
    }

    pub fn load_articles(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        let path = app_dir.join("articles.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(saved) = serde_json::from_str::<Vec<Article>>(&content) {
                    *self.articles.lock().unwrap() = saved;
                }
            }
        }
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_news() -> Vec<NewsItem> {
    vec![
        NewsItem {
            id: 1,
            title: "Rust 1.85 Released".to_string(),
            summary: "New features including async improvements.".to_string(),
            url: "https://blog.rust-lang.org/".to_string(),
        },
        NewsItem {
            id: 2,
            title: "Tauri v2 is Stable".to_string(),
            summary: "Build smaller, faster, and more secure apps.".to_string(),
            url: "https://tauri.app".to_string(),
        },
        NewsItem {
            id: 3,
            title: "Android Native Development".to_string(),
            summary: "Best practices for 2025.".to_string(),
            url: "#".to_string(),
        },
    ]
}

async fn fetch_feed(url: &str, category: ArticleCategory) -> Result<Vec<Article>, String> {
    let content = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let channel = rss::Channel::read_from(Cursor::new(content)).map_err(|e| e.to_string())?;

    // Simple regex to find src="..."
    let re = regex::Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap();

    let articles = channel
        .items()
        .iter()
        .map(|item| {
            // Extract image URL
            let mut image_url = None;

            // 1. Check <enclosure>
            if let Some(enclosure) = item.enclosure() {
                if enclosure.mime_type().starts_with("image") {
                    image_url = Some(enclosure.url().to_string());
                }
            }

            // 2. Check <media:content> (extensions)
            if image_url.is_none() {
                if let Some(media_ext) = item.extensions().get("media") {
                    if let Some(contents) = media_ext.get("content") {
                        if let Some(first_content) = contents.first() {
                            if let Some(url) = first_content.attrs().get("url") {
                                image_url = Some(url.to_string());
                            }
                        }
                    }
                }
            }

            // 3. Regex match <img src="..."> in description or content
            if image_url.is_none() {
                let desc = item.description().unwrap_or("");
                let content = item.content().unwrap_or("");
                if let Some(caps) = re.captures(desc) {
                    image_url = Some(caps[1].to_string());
                } else if let Some(caps) = re.captures(content) {
                    image_url = Some(caps[1].to_string());
                }
            }

            // Extract Author
            let author = item.author().map(|a| a.to_string()).or_else(|| {
                item.dublin_core_ext()
                    .and_then(|dc| dc.creators.first().cloned())
            });

            Article {
                id: item
                    .guid()
                    .map(|g| g.value())
                    .or(item.link())
                    .unwrap_or("")
                    .to_string(),
                title: item.title().unwrap_or("No Title").to_string(),
                summary: item.description().unwrap_or("").chars().take(200).collect(),
                url: item.link().unwrap_or("").to_string(),
                category: category.clone(),
                published_at: item.pub_date().unwrap_or("").to_string(),
                feedback: None,
                image_url,
                author,
            }
        })
        .collect();
    Ok(articles)
}

#[tauri::command]
pub async fn fetch_articles(
    state: State<'_, NewsState>,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    let feeds = vec![
        ("https://blog.rust-lang.org/feed.xml", ArticleCategory::Rust),
        (
            "https://this-week-in-rust.org/rss.xml",
            ArticleCategory::Rust,
        ),
        (
            "https://feeds.feedburner.com/blogspot/hsDu",
            ArticleCategory::Android,
        ),
        ("https://androidweekly.net/rss", ArticleCategory::Android),
        ("https://tauri.app/blog/rss.xml", ArticleCategory::Tauri),
        (
            "https://devblogs.microsoft.com/typescript/feed/",
            ArticleCategory::TypeScript,
        ),
        ("https://react.dev/feed.xml", ArticleCategory::React),
        ("https://overreacted.io/rss.xml", ArticleCategory::React),
    ];

    let mut new_count = 0;

    for (url, category) in feeds {
        if let Ok(fetched) = fetch_feed(url, category).await {
            let mut articles = state.articles.lock().unwrap();
            for item in fetched {
                if !articles.iter().any(|a| a.id == item.id) {
                    articles.push(item);
                    new_count += 1;
                }
            }
        }
    }

    state.save_articles(&app);
    Ok(new_count)
}

async fn recommend_with_gemini(
    candidates: Vec<Article>,
    feedback_history: Vec<Feedback>,
    api_key: String,
) -> Vec<Article> {
    // 1. Construct Prompt
    let mut prompt = String::from("You are a tech article recommender. Select the best 4 articles from the CANDIDATES list based on the USER_FEEDBACK history.\n\n");

    prompt.push_str("USER_FEEDBACK:\n");
    for f in feedback_history.iter().take(20) {
        // Limit history context
        prompt.push_str(&format!(
            "- Helpful: {}, Reason: {}\n",
            f.is_helpful, f.reason
        ));
    }

    prompt.push_str("\nCANDIDATES (JSON):\n");
    // Simplify candidates to save tokens
    let simple_candidates: Vec<_> = candidates
        .iter()
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "title": a.title,
                "category": format!("{:?}", a.category),
                "summary": a.summary.chars().take(100).collect::<String>()
            })
        })
        .collect();
    prompt.push_str(&serde_json::to_string(&simple_candidates).unwrap_or_default());

    prompt.push_str("\n\nRespond ONLY with a JSON array of the IDs of the 4 selected articles. Example: [\"id1\", \"id2\"]");

    // 2. Call Gemini API
    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key))
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()
        .await;

    // 3. Parse Response
    if let Ok(response) = res {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                // Clean markdown code blocks if present
                let clean_text = text
                    .replace("```json", "")
                    .replace("```", "")
                    .trim()
                    .to_string();
                if let Ok(selected_ids) = serde_json::from_str::<Vec<String>>(&clean_text) {
                    return candidates
                        .into_iter()
                        .filter(|a| selected_ids.contains(&a.id))
                        .collect();
                }
            }
        }
    }

    // Fallback: Return top 4 from candidates if AI fails
    candidates.into_iter().take(4).collect()
}

#[tauri::command]
pub async fn get_recommended_articles(state: State<'_, NewsState>) -> Result<Vec<Article>, String> {
    let articles = state.articles.lock().unwrap().clone();

    // 0. Filter out duplicate IDs or bad data? (Assumed clean from fetch)

    // 1. Separate viewed/feedbacked vs new?
    // For now, let's treat all available articles as candidates,
    // but maybe prioritize ones without feedback for the "Rule-based" part if we want "Unread".
    // Or just strictly Date based.

    // Sort by Date Descending
    let mut sorted_articles = articles.clone();
    sorted_articles.sort_by(|a, b| b.published_at.cmp(&a.published_at)); // Dictionary sort of date string works for ISO/RSS standard usually

    // 2. Rule-based: Top 3 (Newest)
    let top_3: Vec<Article> = sorted_articles.iter().take(3).cloned().collect();
    let remaining: Vec<Article> = sorted_articles.iter().skip(3).cloned().collect();

    // 3. AI-based: Next 4 from remaining
    // Get API Key from Env or Settings (Simple Env for now)
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();

    let ai_picks = if !api_key.is_empty() && !remaining.is_empty() {
        // Collect feedback history
        let feedback_history: Vec<Feedback> =
            articles.iter().filter_map(|a| a.feedback.clone()).collect();

        recommend_with_gemini(remaining, feedback_history, api_key).await
    } else {
        // Fallback: Just take next 4
        remaining.into_iter().take(4).collect()
    };

    // 4. Combine
    let mut result = top_3;
    result.extend(ai_picks);

    Ok(result)
}

#[tauri::command]
pub fn submit_feedback(
    id: String,
    helpful: bool,
    reason: String,
    state: State<NewsState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut articles = state.articles.lock().unwrap();
    if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
        article.feedback = Some(Feedback {
            is_helpful: helpful,
            reason,
            created_at: chrono::Local::now().to_rfc3339(),
        });
        // Save immediately
        drop(articles); // release lock before save? save takes lock.
                        // state.save_articles takes &self, and locks.
                        // So I must drop lock here.
        state.save_articles(&app);
        Ok(())
    } else {
        Err("Article not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_news() {
        let news = get_news();
        assert_eq!(news.len(), 3);
        assert_eq!(news[0].title, "Rust 1.85 Released");
    }

    // Additional tests for article logic
    #[test]
    fn test_article_sorting() {
        // Create dummy articles
        let a1 = Article {
            id: "1".to_string(),
            title: "Old".to_string(),
            summary: "".to_string(),
            url: "".to_string(),
            category: ArticleCategory::General,
            published_at: "2024-01-01".to_string(),
            feedback: None,
            image_url: None,
            author: None,
        };
        let a2 = Article {
            id: "2".to_string(),
            title: "New".to_string(),
            summary: "".to_string(),
            url: "".to_string(),
            category: ArticleCategory::General,
            published_at: "2025-01-01".to_string(),
            feedback: None,
            image_url: None,
            author: None,
        };

        let mut articles = vec![a1, a2];
        articles.sort_by(|a, b| b.published_at.cmp(&a.published_at));

        assert_eq!(articles[0].id, "2"); // Newest first
    }
}
