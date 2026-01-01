#[cfg(test)]
mod tests {
    use crate::features::recommendation::model::Article;
    use crate::features::recommendation::model::ArticleCategory;
    use crate::features::recommendation::system::RecommendationState;
    use crate::modules::todo::TodoState;
    use crate::modules::worklog::WorkLogState;
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    fn setup_memory_db() -> Pool<SqliteConnectionManager> {
        // Use shared cache to ensure all connections in the pool see the same in-memory DB
        let manager = SqliteConnectionManager::file("file::memory:?cache=shared");
        let pool = Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();

        // Init tables (copied from db.rs for testing)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                completed BOOLEAN DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .unwrap();

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
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS articles (
                id TEXT PRIMARY KEY,
                title TEXT,
                summary TEXT,
                url TEXT,
                category TEXT,
                published_at TEXT,
                feedback_helpful BOOLEAN NULL,
                feedback_reason TEXT NULL,
                feedback_at TEXT NULL
            )",
            [],
        )
        .unwrap();

        pool
    }

    #[test]
    fn test_todo_crud() {
        let pool = setup_memory_db();
        let state = TodoState::new(pool.clone());

        // Add
        let todos = state.add("Test Todo".to_string()).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Test Todo");
        assert_eq!(todos[0].completed, false);

        let id = todos[0].id;

        // Toggle
        let todos = state.toggle(id).unwrap();
        assert_eq!(todos[0].completed, true);

        // Delete
        let todos = state.delete(id).unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[test]
    fn test_worklog_crud() {
        let pool = setup_memory_db();
        let state = WorkLogState::new(pool.clone());

        // Add
        let logs = state.add("Project X".to_string(), 2.5).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].project, "Project X");
        assert_eq!(logs[0].hours, 2.5);
    }

    #[tokio::test]
    async fn test_articles_db_ops() {
        let pool = setup_memory_db();
        // We can't test fetch_articles fully because it makes HTTP requests.
        // But we can test get_recommended_articles reading from DB if we insert manually.

        let conn = pool.get().unwrap();
        conn.execute(
             "INSERT INTO articles (id, title, summary, url, category, published_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
             rusqlite::params!["1", "Test Article", "Summary", "http://example.com", "Rust", "2023-01-01"],
         ).unwrap();

        let state = RecommendationState::new(pool.clone());

        // Test Get
        let articles = crate::features::recommendation::system::get_recommended_articles(
            tauri::State::from(&state),
        )
        .await
        .unwrap();
        assert!(!articles.is_empty());
        assert_eq!(articles[0].title, "Test Article");
    }
}
