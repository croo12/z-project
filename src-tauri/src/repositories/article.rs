use crate::db::DbPool;
use crate::features::recommendation::model::{Article, ArticleCategory, Feedback};

pub trait ArticleRepository: Send + Sync {
    fn create_bulk(&self, articles: Vec<Article>) -> Result<usize, String>;
    fn get_all(&self) -> Result<Vec<Article>, String>;
    fn get_feedback(&self) -> Result<Vec<Feedback>, String>;
    fn update_feedback(
        &self,
        id: &str,
        helpful: bool,
        reason: &str,
        timestamp: &str,
    ) -> Result<(), String>;
}

pub struct SqliteArticleRepository {
    pool: DbPool,
}

impl SqliteArticleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ArticleRepository for SqliteArticleRepository {
    fn create_bulk(&self, articles: Vec<Article>) -> Result<usize, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut new_count = 0;

        for item in articles {
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

    fn get_all(&self) -> Result<Vec<Article>, String> {
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

    fn get_feedback(&self) -> Result<Vec<Feedback>, String> {
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

    fn update_feedback(
        &self,
        id: &str,
        helpful: bool,
        reason: &str,
        timestamp: &str,
    ) -> Result<(), String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE articles SET feedback_helpful = ?1, feedback_reason = ?2, feedback_at = ?3 WHERE id = ?4",
            rusqlite::params![helpful, reason, timestamp, id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
}
