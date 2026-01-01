pub mod db;
#[cfg(test)]
mod db_tests;
pub mod features;
pub mod modules;

use features::recommendation::system::{
    fetch_articles, get_recommended_articles, submit_feedback, RecommendationState,
};
use modules::{
    todo::{add_todo, delete_todo, get_todos, toggle_todo, TodoState},
    worklog::{add_work_log, get_work_logs, WorkLogState},
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize DB
            let pool = db::init_db(app.handle()).expect("Failed to initialize database");

            // Initialize States
            app.manage(TodoState::new(pool.clone()));
            app.manage(WorkLogState::new(pool.clone()));

            let rec_state = RecommendationState::new(pool.clone());
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
            submit_feedback
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
