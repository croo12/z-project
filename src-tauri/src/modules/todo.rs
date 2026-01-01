use crate::repositories::todo::TodoRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

pub struct TodoState {
    pub repo: Arc<dyn TodoRepository + Send + Sync>,
}

impl TodoState {
    pub fn new(repo: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub fn add(&self, text: String) -> Result<Vec<Todo>, String> {
        self.repo.create(text)?;
        self.get_all()
    }

    pub fn toggle(&self, id: u32) -> Result<Vec<Todo>, String> {
        self.repo.toggle(id)?;
        self.get_all()
    }

    pub fn delete(&self, id: u32) -> Result<Vec<Todo>, String> {
        self.repo.delete(id)?;
        self.get_all()
    }

    pub fn get_all(&self) -> Result<Vec<Todo>, String> {
        self.repo.get_all()
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_todos(state: State<TodoState>) -> Result<Vec<Todo>, String> {
    state.get_all()
}

#[tauri::command]
pub fn add_todo(text: String, state: State<TodoState>) -> Result<Vec<Todo>, String> {
    state.add(text)
}

#[tauri::command]
pub fn toggle_todo(id: u32, state: State<TodoState>) -> Result<Vec<Todo>, String> {
    state.toggle(id)
}

#[tauri::command]
pub fn delete_todo(id: u32, state: State<TodoState>) -> Result<Vec<Todo>, String> {
    state.delete(id)
}
