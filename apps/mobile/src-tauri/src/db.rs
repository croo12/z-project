use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db(app_handle: &AppHandle) -> Result<DbPool, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or(PathBuf::from("."));

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }

    let db_path = app_dir.join("app.db");
    println!("Database path: {:?}", db_path);

    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).map_err(|e| e.to_string())?;

    let conn = pool.get().map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            completed BOOLEAN DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS work_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project TEXT NOT NULL,
            hours REAL NOT NULL,
            date TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (
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
            feedback_at TEXT NULL,
            server_article_id TEXT NULL,
            synced_at TEXT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    // Migration for existing tables
    add_column_if_not_exists(&conn, "articles", "server_article_id", "TEXT NULL")
        .map_err(|e| e.to_string())?;
    add_column_if_not_exists(&conn, "articles", "synced_at", "TEXT NULL")
        .map_err(|e| e.to_string())?;

    // Optimization: Partial index to speed up fetching candidate articles (unread)
    // Most reads filter for `feedback_helpful IS NULL`.
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_feedback ON articles(feedback_helpful) WHERE feedback_helpful IS NULL",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(pool)
}

fn add_column_if_not_exists(
    conn: &rusqlite::Connection,
    table: &str,
    column: &str,
    column_type: &str,
) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let mut rows = stmt.query([])?;
    let mut exists = false;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name == column {
            exists = true;
            break;
        }
    }
    if !exists {
        conn.execute(
            &format!(
                "ALTER TABLE {} ADD COLUMN {} {}",
                table, column, column_type
            ),
            [],
        )?;
    }
    Ok(())
}
