use super::model::{Article, ArticleCategory};
use std::io::Cursor;

pub struct RssParser {
    re_img: regex::Regex,
    re_rust: regex::Regex,
    re_react: regex::Regex,
    re_android: regex::Regex,
    re_tauri: regex::Regex,
    re_ai: regex::Regex,
}

impl Default for RssParser {
    fn default() -> Self {
        Self::new()
    }
}

impl RssParser {
    pub fn new() -> Self {
        Self {
            re_img: regex::Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap(),
            re_rust: regex::Regex::new(r"(?i)\brust\b").unwrap(),
            re_react: regex::Regex::new(r"(?i)\breact\b").unwrap(),
            re_android: regex::Regex::new(r"(?i)\bandroid\b").unwrap(),
            re_tauri: regex::Regex::new(r"(?i)\btauri\b").unwrap(),
            re_ai: regex::Regex::new(r"(?i)\b(ai|llm|gpt|generative)\b").unwrap(),
        }
    }

    pub fn parse(
        &self,
        content: &[u8],
        source_category: ArticleCategory,
    ) -> Result<Vec<Article>, String> {
        let channel = rss::Channel::read_from(Cursor::new(content)).map_err(|e| e.to_string())?;

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
                    if let Some(caps) = self.re_img.captures(desc) {
                        image_url = Some(caps[1].to_string());
                    } else if let Some(caps) = self.re_img.captures(content) {
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
                if self.re_rust.is_match(&text_to_check) && !tags.contains(&ArticleCategory::Rust) {
                    tags.push(ArticleCategory::Rust);
                }
                if self.re_react.is_match(&text_to_check) && !tags.contains(&ArticleCategory::React)
                {
                    tags.push(ArticleCategory::React);
                }
                if self.re_android.is_match(&text_to_check)
                    && !tags.contains(&ArticleCategory::Android)
                {
                    tags.push(ArticleCategory::Android);
                }
                if self.re_tauri.is_match(&text_to_check) && !tags.contains(&ArticleCategory::Tauri)
                {
                    tags.push(ArticleCategory::Tauri);
                }
                if self.re_ai.is_match(&text_to_check) && !tags.contains(&ArticleCategory::AI) {
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
}
