use super::config::{HIGH_IMPACT_KEYWORDS, MEDIUM_IMPACT_KEYWORDS, NEGATIVE_KEYWORDS};
use super::model::{Article, ArticleCategory};
use reqwest;
use std::io::Cursor;

/// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
/// Positive score: Keep/Promote. Negative score: Demote/Discard.
pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
    let mut score = 0;
    // Optimization: Avoid allocating a new concatenated string. Check title and summary individually.
    let title_lower = article.title.to_lowercase();
    let summary_lower = article.summary.to_lowercase();

    for word in HIGH_IMPACT_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
            score += 10;
        }
    }
    for word in MEDIUM_IMPACT_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
            score += 3;
        }
    }
    for word in NEGATIVE_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
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
    client: &reqwest::Client,
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

    // Optimized: Use OnceLock to compile regexes only once
    static RE_IMG: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_img =
        RE_IMG.get_or_init(|| regex::Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap());

    static RE_RUST: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_rust = RE_RUST.get_or_init(|| regex::Regex::new(r"(?i)\brust\b").unwrap());

    static RE_REACT: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_react = RE_REACT.get_or_init(|| regex::Regex::new(r"(?i)\breact\b").unwrap());

    static RE_ANDROID: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_android = RE_ANDROID.get_or_init(|| regex::Regex::new(r"(?i)\bandroid\b").unwrap());

    static RE_TAURI: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_tauri = RE_TAURI.get_or_init(|| regex::Regex::new(r"(?i)\btauri\b").unwrap());

    static RE_AI: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re_ai =
        RE_AI.get_or_init(|| regex::Regex::new(r"(?i)\b(ai|llm|gpt|generative)\b").unwrap());

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
            // Optimization: Truncate content to avoid scanning huge strings
            let desc = item.description().unwrap_or("");
            let content = item.content().unwrap_or("");

            // Use char_indices to find the byte offset for the char limit without allocating new strings
            let desc_limit = desc
                .char_indices()
                .map(|(i, _)| i)
                .nth(5000)
                .unwrap_or(desc.len());
            let desc_trunc = &desc[..desc_limit];

            let content_limit = content
                .char_indices()
                .map(|(i, _)| i)
                .nth(5000)
                .unwrap_or(content.len());
            let content_trunc = &content[..content_limit];

            if image_url.is_none() {
                if let Some(caps) = re_img.captures(desc_trunc) {
                    image_url = Some(caps[1].to_string());
                } else if let Some(caps) = re_img.captures(content_trunc) {
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
            let text_to_check = format!("{} {}", title, desc_trunc);

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
