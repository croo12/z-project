use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ArticleCategory {
    Rust,
    General,
}

#[derive(Debug)]
struct Article {
    id: String,
    title: String,
    feedback_helpful: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE articles (
            id TEXT PRIMARY KEY,
            title TEXT,
            summary TEXT,
            url TEXT UNIQUE,
            tags TEXT,
            published_at TEXT,
            image_url TEXT NULL,
            author TEXT NULL,
            feedback_helpful BOOLEAN NULL,
            feedback_reason TEXT NULL,
            feedback_at TEXT NULL
        )",
        [],
    )?;

    // Insert Article 1: Unread (No feedback)
    conn.execute(
        "INSERT INTO articles (id, title, summary, url, tags, published_at, feedback_helpful)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            "1",
            "Unread Article",
            "Summary",
            "http://1",
            "[\"Rust\"]",
            "2023-01-01",
            None::<bool>
        ],
    )?;

    // Insert Article 2: Read (Has feedback)
    conn.execute(
        "INSERT INTO articles (id, title, summary, url, tags, published_at, feedback_helpful)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            "2",
            "Read Article",
            "Summary",
            "http://2",
            "[\"General\"]",
            "2023-01-01",
            Some(true)
        ],
    )?;

    // Proposed Query
    let mut stmt = conn.prepare("SELECT id, title, feedback_helpful FROM articles WHERE feedback_helpful IS NULL")?;

    let articles_iter = stmt.query_map([], |row| {
        Ok(Article {
            id: row.get(0)?,
            title: row.get(1)?,
            feedback_helpful: row.get(2)?,
        })
    })?;

    let mut articles = Vec::new();
    for a in articles_iter {
        articles.push(a?);
    }

    println!("Found articles: {:?}", articles);

    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].id, "1");

    println!("Verification passed!");
    Ok(())
}
