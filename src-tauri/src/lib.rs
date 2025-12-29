use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Clone)]
struct NewsItem {
    id: u32,
    title: String,
    summary: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct WorkLog {
    id: u32,
    project: String,
    hours: f32,
    date: String,
}

// --- State Management (Simple In-Memory for Demo) ---

struct AppState {
    todos: Mutex<Vec<TodoItem>>,
    work_logs: Mutex<Vec<WorkLog>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: Mutex::new(vec![
                TodoItem {
                    id: 1,
                    text: "Install Android Studio".to_string(),
                    completed: false,
                },
                TodoItem {
                    id: 2,
                    text: "Learn Rust".to_string(),
                    completed: true,
                },
            ]),
            work_logs: Mutex::new(vec![WorkLog {
                id: 1,
                project: "Personal App".to_string(),
                hours: 2.5,
                date: "2025-12-29".to_string(),
            }]),
        }
    }

    fn add_todo(&self, text: String) -> Vec<TodoItem> {
        let mut todos = self.todos.lock().unwrap();
        let id = todos.len() as u32 + 1;
        todos.push(TodoItem {
            id,
            text,
            completed: false,
        });
        todos.clone()
    }

    fn toggle_todo(&self, id: u32) -> Vec<TodoItem> {
        let mut todos = self.todos.lock().unwrap();
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        todos.clone()
    }

    fn add_work_log(&self, project: String, hours: f32) -> Vec<WorkLog> {
        let mut logs = self.work_logs.lock().unwrap();
        let id = logs.len() as u32 + 1;
        let date = "2025-12-29".to_string();
        logs.push(WorkLog {
            id,
            project,
            hours,
            date,
        });
        logs.clone()
    }
}

// --- Commands ---

#[tauri::command]
fn get_news() -> Vec<NewsItem> {
    vec![
        NewsItem {
            id: 1,
            title: "Rust 1.85 Released".to_string(),
            summary: "New features including async improvements.".to_string(),
            url: "https://blog.rust-lang.org/".to_string(),
        },
        NewsItem {
            id: 2,
            title: "Tauri v2 is Stable".to_string(),
            summary: "Build smaller, faster, and more secure apps.".to_string(),
            url: "https://tauri.app".to_string(),
        },
        NewsItem {
            id: 3,
            title: "Android Native Development".to_string(),
            summary: "Best practices for 2025.".to_string(),
            url: "#".to_string(),
        },
    ]
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<TodoItem> {
    let todos = state.todos.lock().unwrap();
    todos.clone()
}

#[tauri::command]
fn add_todo(text: String, state: State<AppState>) -> Vec<TodoItem> {
    state.add_todo(text)
}

#[tauri::command]
fn toggle_todo(id: u32, state: State<AppState>) -> Vec<TodoItem> {
    state.toggle_todo(id)
}

#[tauri::command]
fn get_work_logs(state: State<AppState>) -> Vec<WorkLog> {
    let logs = state.work_logs.lock().unwrap();
    logs.clone()
}

#[tauri::command]
fn add_work_log(project: String, hours: f32, state: State<AppState>) -> Vec<WorkLog> {
    state.add_work_log(project, hours)
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let state = AppState::new();
        let todos = state.add_todo("Buy milk".to_string());
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Buy milk");
        assert_eq!(todos[0].completed, false);
    }

    #[test]
    fn test_toggle_todo() {
        let state = AppState::new();
        state.add_todo("Learn Rust".to_string());

        // Toggle logic
        let todos = state.toggle_todo(1);
        assert_eq!(todos[0].completed, true);

        // Toggle back
        let todos = state.toggle_todo(1);
        assert_eq!(todos[0].completed, false);
    }

    #[test]
    fn test_add_work_log() {
        let state = AppState::new();
        let logs = state.add_work_log("Tauri App".to_string(), 4.5);
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].project, "Tauri App");
        assert_eq!(logs[0].hours, 4.5);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            get_news,
            get_todos,
            add_todo,
            toggle_todo,
            get_work_logs,
            add_work_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
