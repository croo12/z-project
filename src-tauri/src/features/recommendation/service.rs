use super::model::{Article, ArticleCategory, Feedback, UserPersona};
use reqwest;
use std::io::Cursor;
use std::sync::OnceLock;

static RE_IMG: OnceLock<regex::Regex> = OnceLock::new();
static RE_RUST: OnceLock<regex::Regex> = OnceLock::new();
static RE_REACT: OnceLock<regex::Regex> = OnceLock::new();
static RE_ANDROID: OnceLock<regex::Regex> = OnceLock::new();
static RE_TAURI: OnceLock<regex::Regex> = OnceLock::new();
static RE_AI: OnceLock<regex::Regex> = OnceLock::new();

/// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
/// Positive score: Keep/Promote. Negative score: Demote/Discard.
pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
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
        "blockchain",
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

    // Category Bonus using Tags
    for tag in &article.tags {
        // 1. Explicit User Interest Bonus (Primary Filter)
        if user_interests.contains(tag) {
            score += 50; // Huge boost for explicit selection
        }

        // 2. General Tech Bonus
        match tag {
            ArticleCategory::Rust
            | ArticleCategory::Tauri
            | ArticleCategory::React
            | ArticleCategory::Android => {
                score += 5;
            }
            ArticleCategory::General => {
                // No bonus
            }
            _ => {
                score += 2;
            }
        }
    }

    // Feedback Logic (User Override)
    // If feedback exists (Positive or Negative), consider it "Read/Processed" and remove from recommendations.
    if article.feedback.is_some() {
        score -= 1000;
    }

    score
}

pub async fn fetch_feed(
    url: &str,
    source_category: ArticleCategory,
) -> Result<Vec<Article>, String> {
    let content = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let channel = rss::Channel::read_from(Cursor::new(content)).map_err(|e| e.to_string())?;

    // Simple regex to find src="..."
    let re_img = RE_IMG.get_or_init(|| regex::Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap());

    // Keyword Regexes for Re-classification
    let re_rust = RE_RUST.get_or_init(|| regex::Regex::new(r"(?i)\brust\b").unwrap());
    let re_react = RE_REACT.get_or_init(|| regex::Regex::new(r"(?i)\breact\b").unwrap());
    let re_android = RE_ANDROID.get_or_init(|| regex::Regex::new(r"(?i)\bandroid\b").unwrap());
    let re_tauri = RE_TAURI.get_or_init(|| regex::Regex::new(r"(?i)\btauri\b").unwrap());
    let re_ai = RE_AI.get_or_init(|| regex::Regex::new(r"(?i)\b(ai|llm|gpt|generative)\b").unwrap());

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
            let desc = item.description().unwrap_or("");
            let content = item.content().unwrap_or("");

            if image_url.is_none() {
                if let Some(caps) = re_img.captures(desc) {
                    image_url = Some(caps[1].to_string());
                } else if let Some(caps) = re_img.captures(content) {
                    image_url = Some(caps[1].to_string());
                }
            }

            // Extract Author
            let author = item.author().map(|a| a.to_string()).or_else(|| {
                item.dublin_core_ext()
                    .and_then(|dc| dc.creators.first().cloned())
            });

            // Tags Logic
            let mut tags = vec![source_category.clone()];
            let title = item.title().unwrap_or("");
            let text_to_check = format!("{} {}", title, desc);

            // Keyword based expansion
            if re_rust.is_match(&text_to_check) && !tags.contains(&ArticleCategory::Rust) {
                tags.push(ArticleCategory::Rust);
            }
            if re_react.is_match(&text_to_check) && !tags.contains(&ArticleCategory::React) {
                tags.push(ArticleCategory::React);
            }
            if re_android.is_match(&text_to_check) && !tags.contains(&ArticleCategory::Android) {
                tags.push(ArticleCategory::Android);
            }
            if re_tauri.is_match(&text_to_check) && !tags.contains(&ArticleCategory::Tauri) {
                tags.push(ArticleCategory::Tauri);
            }
            if re_ai.is_match(&text_to_check) && !tags.contains(&ArticleCategory::AI) {
                tags.push(ArticleCategory::AI);
            }

            // Remove General if specialized tag exists
            if tags.len() > 1 && tags[0] == ArticleCategory::General {
                tags.remove(0);
            }

            Article {
                id: item
                    .guid()
                    .map(|g| g.value())
                    .or(item.link())
                    .unwrap_or("")
                    .to_string(),
                title: title.to_string(),
                summary: desc.chars().take(250).collect(),
                url: item.link().unwrap_or("").to_string(),
                tags,
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

    prompt.push_str("\nTask: Analyze the feedback patterns to refine the User Persona.\n");
    prompt.push_str("INSTRUCTIONS:\n");
    prompt.push_str(
        "1. Identify specific keywords or topics the user explicitly LIKES (Helpful=true).\n",
    );
    prompt.push_str("2. Identify topics the user DISLIKES (Helpful=false).\n");
    prompt.push_str("3. Update the description to be specific (e.g., 'User prefers Rust async and Tauri architecture, but dislikes general finance news').\n");
    prompt.push_str("4. Output ONLY the concise description text (2-3 sentences).");

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
    user_interests: &[ArticleCategory],
    api_key: String,
) -> Vec<Article> {
    // 1. Construct Prompt
    let mut prompt = String::from("You are a tech article recommender. Select the best 4 articles from the CANDIDATES list.\n\n");

    // Explicit Inputs
    if !user_interests.is_empty() {
        prompt.push_str(&format!("USER SELECTED TAGS: {:?}\n", user_interests));
        prompt.push_str("INSTRUCTION: Prioritize articles that match the USER SELECTED TAGS above all else.\n\n");
    }

    if !persona.description.is_empty() {
        prompt.push_str(&format!("USER PERSONA (Implicit Preferences):\n{}\n\nThen, refine the selection to match this persona.\n\n", persona.description));
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
                "tags": format!("{:?}", a.tags),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::recommendation::model::{Article, ArticleCategory, Feedback};

    #[test]
    fn test_feedback_scoring_internal() {
        // Case: Downvoted article
        let downvoted_article = Article {
            id: "down".into(),
            title: "Bad Article".into(),
            summary: "Not helpful".into(),
            url: "http://bad.com".into(),
            tags: vec![ArticleCategory::Rust],
            published_at: "".into(),
            feedback: Some(Feedback {
                is_helpful: false,
                reason: "Bad".into(),
                created_at: "".into(),
            }),
            image_url: None,
            author: None,
        };

        // Case: Upvoted (Already Read) article
        let upvoted_article = Article {
            id: "up".into(),
            title: "Good Article".into(),
            summary: "Helpful".into(),
            url: "http://good.com".into(),
            tags: vec![ArticleCategory::Rust],
            published_at: "".into(),
            feedback: Some(Feedback {
                is_helpful: true,
                reason: "Good".into(),
                created_at: "".into(),
            }),
            image_url: None,
            author: None,
        };

        let s1 = calculate_relevance_score(&downvoted_article, &[]);
        let s2 = calculate_relevance_score(&upvoted_article, &[]);

        assert!(
            s1 < -500,
            "Downvoted article should be buried (-1000 penalty)"
        );
        assert!(
            s2 < -500,
            "Upvoted article should also be hidden (treated as read)"
        );
    }
}
