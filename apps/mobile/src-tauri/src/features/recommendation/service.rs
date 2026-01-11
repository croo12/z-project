use super::model::{Article, ArticleCategory};
use reqwest;
use std::io::Cursor;

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

            // Optimization: Avoid allocating a new string with format! by checking title and desc separately.
            // This is safe because all regexes use \b word boundaries or don't span words.
            let check_keyword = |re: &regex::Regex| -> bool {
                re.is_match(title) || re.is_match(desc_trunc)
            };

            // Keyword based expansion
            if check_keyword(re_rust) && !tags.contains(&ArticleCategory::Rust) {
                tags.push(ArticleCategory::Rust);
            }
            if check_keyword(re_react) && !tags.contains(&ArticleCategory::React) {
                tags.push(ArticleCategory::React);
            }
            if check_keyword(re_android) && !tags.contains(&ArticleCategory::Android) {
                tags.push(ArticleCategory::Android);
            }
            if check_keyword(re_tauri) && !tags.contains(&ArticleCategory::Tauri) {
                tags.push(ArticleCategory::Tauri);
            }
            if check_keyword(re_ai) && !tags.contains(&ArticleCategory::AI) {
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
