pub mod modules;

use modules::{
    news::{fetch_articles, get_news, get_recommended_articles, submit_feedback, NewsState},
    todo::{add_todo, get_todos, toggle_todo, TodoState},
    worklog::{add_work_log, get_work_logs, WorkLogState},
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(TodoState::with_demo_data())
        .manage(WorkLogState::with_demo_data())
        .manage(NewsState::default())
        .invoke_handler(tauri::generate_handler![
            get_news,
            get_todos,
            add_todo,
            toggle_todo,
            get_work_logs,
            add_work_log,
            fetch_articles,
            get_recommended_articles,
            submit_feedback
        ])
        .setup(|app| {
            let news_state = app.state::<NewsState>();
            news_state.load_articles(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
