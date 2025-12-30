use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tauri::Manager;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ArticleCategory {
    React,
    Rust,
    Android,
    Tauri,
    TypeScript,
    General,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feedback {
    pub is_helpful: bool,
    pub reason: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub url: String,
    pub category: ArticleCategory,
    pub published_at: String,
    pub feedback: Option<Feedback>,
}

// --- State Management (Simple In-Memory for Demo) ---

struct AppState {
    todos: Mutex<Vec<TodoItem>>,
    work_logs: Mutex<Vec<WorkLog>>,
    articles: Mutex<Vec<Article>>,
}

impl AppState {
    #[cfg(test)]
    fn new() -> Self {
        Self {
            todos: Mutex::new(Vec::new()),
            work_logs: Mutex::new(Vec::new()),
            articles: Mutex::new(Vec::new()),
        }
    }

    fn with_demo_data() -> Self {
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
            articles: Mutex::new(Vec::new()),
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

    fn save_articles(&self, app_handle: &tauri::AppHandle) {
        let articles = self.articles.lock().unwrap();
        let app_dir = app_handle.path().app_data_dir().unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let path = app_dir.join("articles.json");
        let _ = fs::write(path, serde_json::to_string(&*articles).unwrap_or_default());
    }

    fn load_articles(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle.path().app_data_dir().unwrap_or(PathBuf::from("."));
        let path = app_dir.join("articles.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(saved) = serde_json::from_str::<Vec<Article>>(&content) {
                    *self.articles.lock().unwrap() = saved;
                }
            }
        }
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

// --- Article Commands ---

async fn fetch_feed(url: &str, category: ArticleCategory) -> Result<Vec<Article>, String> {
    let content = reqwest::get(url).await.map_err(|e| e.to_string())?
        .bytes().await.map_err(|e| e.to_string())?;
    let channel = rss::Channel::read_from(Cursor::new(content)).map_err(|e| e.to_string())?;
    
    let articles = channel.items().into_iter().map(|item| {
        Article {
            id: item.guid().map(|g| g.value()).or(item.link()).unwrap_or("").to_string(),
            title: item.title().unwrap_or("No Title").to_string(),
            summary: item.description().unwrap_or("").chars().take(200).collect(),
            url: item.link().unwrap_or("").to_string(),
            category: category.clone(),
            published_at: item.pub_date().unwrap_or("").to_string(),
            feedback: None,
        }
    }).collect();
    Ok(articles)
}

#[tauri::command]
async fn fetch_articles(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<usize, String> {
    let feeds = vec![
        ("https://blog.rust-lang.org/feed.xml", ArticleCategory::Rust),
        ("https://feeds.feedburner.com/blogspot/hsDu", ArticleCategory::Android),
        ("https://tauri.app/blog/rss.xml", ArticleCategory::Tauri),
        ("https://devblogs.microsoft.com/typescript/feed/", ArticleCategory::TypeScript),
        // React feed is tricky, leaving out or using a known working one if possible. 
        // Using a generic one or skipping to avoid noise if unsure. 
        // Spec said https://react.dev/feed.xml. I'll try it.
        ("https://react.dev/feed.xml", ArticleCategory::React), 
    ];

    let mut new_count = 0;
    
    // In a real app, use join_all to fetch concurrently. 
    // Here sequential is fine for a demo commands.
    for (url, category) in feeds {
        if let Ok(fetched) = fetch_feed(url, category).await {
            let mut articles = state.articles.lock().unwrap();
            for item in fetched {
                if !articles.iter().any(|a| a.id == item.id) {
                    articles.push(item);
                    new_count += 1;
                }
            }
        }
    }
    
    state.save_articles(&app);
    Ok(new_count)
}

#[tauri::command]
fn get_recommended_articles(state: State<AppState>) -> Vec<Article> {
    let articles = state.articles.lock().unwrap();
    // Simple sort by date (string compare approx works for standard formats, but not perfect) 
    // and put unread/helpful on top? 
    // For V1, just return allreversed (newest first usually in RSS, but we appended)
    // Actually we appended so newest might be at end if feed is old->new? 
    // RSS items usually new->old.
    // Let's just reverse list.
    let mut list = articles.clone();
    list.reverse(); 
    list
}

#[tauri::command]
fn submit_feedback(id: String, helpful: bool, reason: String, state: State<AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let mut articles = state.articles.lock().unwrap();
    if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
        article.feedback = Some(Feedback {
            is_helpful: helpful,
            reason,
            created_at: chrono::Local::now().to_rfc3339(),
        });
        // Save immediately
        drop(articles); // release lock before save? save takes lock. 
        // state.save_articles takes &self, and locks. 
        // So I must drop lock here.
        state.save_articles(&app);
        Ok(())
    } else {
        Err("Article not found".to_string())
    }
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
        .manage(AppState::with_demo_data())
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
            let state = app.state::<AppState>();
            state.load_articles(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
