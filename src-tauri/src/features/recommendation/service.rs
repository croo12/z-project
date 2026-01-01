use super::model::{Article, ArticleCategory, Feedback, UserPersona};
use reqwest;
use std::io::Cursor;

/// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
/// Positive score: Keep/Promote. Negative score: Demote/Discard.
pub fn calculate_relevance_score(article: &Article) -> i32 {
    let mut score = 0;
    let title_lower = article.title.to_lowercase();
    let summary_lower = article.summary.to_lowercase();
    let content_to_check = format!("{} {}", title_lower, summary_lower);

    // High Impact Keywords
    let high_impact = [
        "rust",
        "tauri",
        "react",
        "typescript",
        "javascript",
        "android",
        "kotlin",
        "webassembly",
        "wasm",
        "docker",
        "kubernetes",
        "llvm",
        "compiler",
    ];
    // Medium Impact Keywords
    let medium_impact = [
        "code",
        "programming",
        "developer",
        "api",
        "frontend",
        "backend",
        "database",
        "algorithm",
        "git",
        "linux",
        "windows",
        "macos",
        "design pattern",
        "refactoring",
    ];
    // Negative Keywords (Noise Filter)
    let negative = [
        "stock",
        "market",
        "buffett",
        "berkshire",
        "invest",
        "politics",
        "crime",
        "murder",
        "sport",
        "celebrity",
        "gossip",
        "bitcoin",
        "crypto",
        "blockchain", // Note: Crypto/Blockchain is debatable for tech, but often just noise/finance.
                      // Keeping it partial negative or neutral might be better, but 'bitcoin' usually means price.
    ];

    for word in high_impact.iter() {
        if content_to_check.contains(word) {
            score += 10;
        }
    }
    for word in medium_impact.iter() {
        if content_to_check.contains(word) {
            score += 3;
        }
    }
    for word in negative.iter() {
        if content_to_check.contains(word) {
            score -= 20; // Strong penalty
        }
    }

    // Category Bonus
    match article.category {
        ArticleCategory::Rust
        | ArticleCategory::Tauri
        | ArticleCategory::React
        | ArticleCategory::Android => {
            score += 5;
        }
        ArticleCategory::General => {
            // No bonus, rely on content
        }
        _ => {
            score += 2;
        }
    }

    score
}

pub async fn fetch_feed(
    client: &reqwest::Client,
    url: &str,
    category: ArticleCategory,
) -> Result<Vec<Article>, String> {
    let content = client
        .get(url)
        .send()
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
                summary: item.description().unwrap_or("").chars().take(250).collect(),
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

pub async fn update_user_persona(
    feedback_history: &[Feedback],
    current_persona: &UserPersona,
    api_key: &str,
) -> Result<UserPersona, String> {
    if feedback_history.is_empty() {
        return Ok(current_persona.clone());
    }

    let mut prompt = String::from("You are an expert user analyst. Update the User Persona based on the recent feedback provided.\n\n");

    if !current_persona.description.is_empty() {
        prompt.push_str(&format!(
            "CURRENT PERSONA:\n{}\n\n",
            current_persona.description
        ));
    }

    prompt.push_str("RECENT FEEDBACK:\n");
    for f in feedback_history.iter().take(20) {
        // Analyze last 20 feedback items
        prompt.push_str(&format!(
            "- Helpful: {}, Reason: {}\n",
            f.is_helpful, f.reason
        ));
    }

    prompt.push_str("\nTask: Write a concise (2-3 sentences) description of this user's technical interests and content preferences based entirely on the verified feedback. Be specific (e.g., 'User likes Rust async, dislikes general finance'). Avoid generic statements.\n\nOutput ONLY the description text.");

    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}", api_key))
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if let Ok(json) = res.json::<serde_json::Value>().await {
        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            return Ok(UserPersona {
                description: text.trim().to_string(),
                last_updated: chrono::Local::now().to_rfc3339(),
            });
        }
    }

    Err("Failed to generate persona".to_string())
}

pub async fn recommend_with_gemini(
    candidates: Vec<Article>,
    persona: &UserPersona,
    api_key: String,
) -> Vec<Article> {
    // 1. Construct Prompt
    let mut prompt = String::from("You are a tech article recommender. Select the best 4 articles from the CANDIDATES list.\n\n");

    if !persona.description.is_empty() {
        prompt.push_str(&format!("TARGET USER PROFILE:\n{}\n\nSelect articles that strictly match this user profile.\n\n", persona.description));
    } else {
        prompt.push_str("Prioritize technical depth and relevance to Rust, Tauri, React, and System Programming.\n\n");
    }

    prompt.push_str("CANDIDATES (JSON):\n");
    let simple_candidates: Vec<_> = candidates
        .iter()
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "title": a.title,
                "category": format!("{:?}", a.category),
                "summary": a.summary.chars().take(150).collect::<String>()
            })
        })
        .collect();
    prompt.push_str(&serde_json::to_string(&simple_candidates).unwrap_or_default());

    prompt.push_str("\n\nRespond ONLY with a JSON array of the IDs of the 4 selected articles.");

    // 2. Call Gemini API
    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}", api_key))
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

    // Fallback or if AI fails
    candidates.into_iter().take(4).collect()
}
