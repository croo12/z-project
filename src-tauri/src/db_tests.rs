#[cfg(test)]
mod tests {
    use crate::features::recommendation::system::RecommendationState;
    use crate::modules::todo::TodoState;
    use crate::modules::worklog::WorkLogState;
    use crate::repositories::article::SqliteArticleRepository;
    use crate::repositories::todo::SqliteTodoRepository;
    use crate::repositories::worklog::SqliteWorkLogRepository;
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use std::sync::Arc;

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
        let repo = Arc::new(SqliteTodoRepository::new(pool.clone()));
        let state = TodoState::new(repo);

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
        let repo = Arc::new(SqliteWorkLogRepository::new(pool.clone()));
        let state = WorkLogState::new(repo);

        // Add
        state
            .repo
            .create("Project X".to_string(), 2.5)
            .unwrap();
        let logs = state.repo.get_all().unwrap();

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].project, "Project X");
        assert_eq!(logs[0].hours, 2.5);
    }

    #[test]
    fn test_articles_db_ops() {
        // Changed to synchronous test since we refactored the logic to be synchronous helper methods.
        // This avoids async/tokio complexity in unit tests for pure DB logic.

        let pool = setup_memory_db();
        let conn = pool.get().unwrap();
        conn.execute(
             "INSERT INTO articles (id, title, summary, url, category, published_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
             rusqlite::params!["1", "Test Article", "Summary", "http://example.com", "Rust", "2023-01-01"],
         ).unwrap();

        let repo = Arc::new(SqliteArticleRepository::new(pool.clone()));
        let state = RecommendationState::new(repo);

        // Test Get (Internal DB method via repository)
        let articles = state.repository.get_all().unwrap();
        assert!(!articles.is_empty());
        assert_eq!(articles[0].title, "Test Article");
    }
}
