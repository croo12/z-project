pub mod db;
#[cfg(test)]
mod db_tests;
pub mod error;
pub mod features;

// Re-exports for easier access if needed, or update consumers to use features::*
use features::recommendation::{
    commands::{
        fetch_articles, get_recommended_articles, get_user_interests, save_user_interests,
        submit_feedback,
    },
    repository::SqliteRecommendationRepository,
    system::RecommendationState,
};
use features::todo::{
    commands::{add_todo, delete_todo, get_todos, toggle_todo},
    repository::SqliteTodoRepository,
    service::TodoState,
};
use features::worklog::{
    commands::{add_work_log, get_work_logs},
    repository::SqliteWorkLogRepository,
    service::WorkLogState,
};

use features::sync::{check_server_health, sync_article_to_server};

#[cfg(debug_assertions)]
use features::debug::commands::test_ai_connection;

use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_app::init())
        .setup(|app| {
            // Initialize DB
            let pool = db::init_db(app.handle()).expect("Failed to initialize database");

            // Initialize Repositories
            let todo_repo = Arc::new(SqliteTodoRepository::new(pool.clone()));
            let worklog_repo = Arc::new(SqliteWorkLogRepository::new(pool.clone()));

            // Initialize States
            app.manage(TodoState::new(todo_repo));
            app.manage(WorkLogState::new(worklog_repo));

            let rec_repo = Arc::new(SqliteRecommendationRepository::new(pool.clone()));
            let rec_state = RecommendationState::new(rec_repo);
            // Load Persona (JSON)
            rec_state.load_persona(app.handle());
            app.manage(rec_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo,
            get_work_logs,
            add_work_log,
            fetch_articles,
            get_recommended_articles,
            submit_feedback,
            save_user_interests,
            get_user_interests,
            sync_article_to_server,
            check_server_health,
            #[cfg(debug_assertions)]
            test_ai_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
