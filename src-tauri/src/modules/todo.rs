use crate::db::DbPool;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

pub struct TodoState {
    pub pool: DbPool,
}

impl TodoState {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn add(&self, text: String) -> Result<Vec<Todo>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO todos (text, completed) VALUES (?1, 0)",
            [&text],
        )
        .map_err(|e| e.to_string())?;
        self.get_all()
    }

    pub fn toggle(&self, id: u32) -> Result<Vec<Todo>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        // First get current status
        let completed: bool = conn
            .query_row("SELECT completed FROM todos WHERE id = ?1", [id], |row| {
                row.get(0)
            })
            .map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE todos SET completed = ?1 WHERE id = ?2",
            (!completed, id),
        )
        .map_err(|e| e.to_string())?;

        self.get_all()
    }

    pub fn delete(&self, id: u32) -> Result<Vec<Todo>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM todos WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        self.get_all()
    }

    pub fn get_all(&self) -> Result<Vec<Todo>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, text, completed FROM todos ORDER BY id ASC")
            .map_err(|e| e.to_string())?;

        let todo_iter = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    completed: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo.map_err(|e| e.to_string())?);
        }
        Ok(todos)
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
