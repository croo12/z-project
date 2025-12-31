use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

pub struct TodoState {
    pub todos: Mutex<Vec<TodoItem>>,
}

impl Default for TodoState {
    fn default() -> Self {
        Self {
            todos: Mutex::new(Vec::new()),
        }
    }
}

impl TodoState {
    pub fn with_demo_data() -> Self {
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
        }
    }

    pub fn add(&self, text: String) -> Vec<TodoItem> {
        let mut todos = self.todos.lock().unwrap();
        let id = todos.len() as u32 + 1;
        todos.push(TodoItem {
            id,
            text,
            completed: false,
        });
        todos.clone()
    }

    pub fn toggle(&self, id: u32) -> Vec<TodoItem> {
        let mut todos = self.todos.lock().unwrap();
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        todos.clone()
    }

    pub fn get_all(&self) -> Vec<TodoItem> {
        self.todos.lock().unwrap().clone()
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_todos(state: State<TodoState>) -> Vec<TodoItem> {
    state.get_all()
}

#[tauri::command]
pub fn add_todo(text: String, state: State<TodoState>) -> Vec<TodoItem> {
    state.add(text)
}

#[tauri::command]
pub fn toggle_todo(id: u32, state: State<TodoState>) -> Vec<TodoItem> {
    state.toggle(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let state = TodoState::default();
        let todos = state.add("Buy milk".to_string());
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].text, "Buy milk");
        assert_eq!(todos[0].completed, false);
    }

    #[test]
    fn test_toggle_todo() {
        let state = TodoState::default();
        state.add("Learn Rust".to_string());

        // Toggle logic
        let todos = state.toggle(1);
        assert_eq!(todos[0].completed, true);

        // Toggle back
        let todos = state.toggle(1);
        assert_eq!(todos[0].completed, false);
    }

    #[test]
    fn test_get_todos() {
        let state = TodoState::with_demo_data();
        let todos = state.get_all();
        assert_eq!(todos.len(), 2);
    }
}
