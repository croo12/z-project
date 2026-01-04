use std::sync::Arc;
use crate::db::DbPool;
use crate::features::recommendation::model::{Article, ArticleCategory, Feedback};
use crate::error::AppError;
use rusqlite::OptionalExtension;

pub trait RecommendationRepository: Send + Sync {
    fn get_articles(&self) -> Result<Vec<Article>, AppError>;
    fn get_feedback(&self) -> Result<Vec<Feedback>, AppError>;
    fn check_article_exists(&self, url: &str) -> Result<Option<String>, AppError>;
    fn save_article(&self, article: Article) -> Result<(), AppError>;
    fn update_feedback(&self, id: &str, helpful: bool, reason: &str, timestamp: &str) -> Result<(), AppError>;
    fn get_feedback_count(&self) -> Result<i64, AppError>;
}

pub struct SqliteRecommendationRepository {
    pool: DbPool,
}

impl SqliteRecommendationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RecommendationRepository for SqliteRecommendationRepository {
    fn get_articles(&self) -> Result<Vec<Article>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT id, title, summary, url, tags, published_at, image_url, author, feedback_helpful, feedback_reason, feedback_at FROM articles")?;

        let articles_iter = stmt.query_map([], |row| {
            let tags_str: String = row.get(4)?;
            let tags: Vec<ArticleCategory> = serde_json::from_str(&tags_str)?;

            let image_url: Option<String> = row.get(6).ok();
            let author: Option<String> = row.get(7).ok();
            let feedback_helpful: Option<bool> = row.get(8).ok();
            let feedback_reason: Option<String> = row.get(9).ok();
            let feedback_at: Option<String> = row.get(10).ok();

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
                tags,
                published_at: row.get(5)?,
                image_url,
                author,
                feedback,
            })
        })?;

        let mut articles = Vec::new();
        for a in articles_iter {
            articles.push(a?);
        }
        Ok(articles)
    }

    fn get_feedback(&self) -> Result<Vec<Feedback>, AppError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT feedback_helpful, feedback_reason, feedback_at FROM articles WHERE feedback_helpful IS NOT NULL")?;

        let feedback_iter = stmt.query_map([], |row| {
            Ok(Feedback {
                is_helpful: row.get(0)?,
                reason: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        let mut feedbacks = Vec::new();
        for f in feedback_iter {
            feedbacks.push(f?);
        }
        Ok(feedbacks)
    }

    fn check_article_exists(&self, url: &str) -> Result<Option<String>, AppError> {
        let conn = self.pool.get()?;
        let tags_json: Option<String> = conn
            .query_row(
                "SELECT tags FROM articles WHERE url = ?1",
                rusqlite::params![url],
                |row| row.get(0),
            )
            .optional()?;
        Ok(tags_json)
    }

    fn save_article(&self, item: Article) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        let tags_json = serde_json::to_string(&item.tags).unwrap_or("[]".to_string());

        conn.execute(
            "INSERT INTO articles (id, title, summary, url, tags, published_at, image_url, author, feedback_helpful, feedback_reason, feedback_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(url) DO UPDATE SET tags = ?5, published_at = ?6, image_url = ?7, author = ?8",
            rusqlite::params![
                item.id,
                item.title,
                item.summary,
                item.url,
                tags_json,
                item.published_at,
                item.image_url,
                item.author,
                item.feedback.as_ref().map(|f| f.is_helpful),
                item.feedback.as_ref().map(|f| f.reason.clone()),
                item.feedback.as_ref().map(|f| f.created_at.clone())
            ],
        )?;
        Ok(())
    }

    fn update_feedback(&self, id: &str, helpful: bool, reason: &str, timestamp: &str) -> Result<(), AppError> {
        let conn = self.pool.get()?;
        conn.execute(
            "UPDATE articles SET feedback_helpful = ?1, feedback_reason = ?2, feedback_at = ?3 WHERE id = ?4",
            rusqlite::params![helpful, reason, timestamp, id],
        )?;
        Ok(())
    }

    fn get_feedback_count(&self) -> Result<i64, AppError> {
        let conn = self.pool.get()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM articles WHERE feedback_helpful IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
