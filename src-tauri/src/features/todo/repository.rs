use super::model::Todo;
use crate::db::DbPool;

pub trait TodoRepository {
    fn get_all(&self) -> Result<Vec<Todo>, String>;
    fn create(&self, text: String) -> Result<(), String>;
    fn toggle(&self, id: u32) -> Result<(), String>;
    fn delete(&self, id: u32) -> Result<(), String>;
}

pub struct SqliteTodoRepository {
    pool: DbPool,
}

impl SqliteTodoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TodoRepository for SqliteTodoRepository {
    fn get_all(&self) -> Result<Vec<Todo>, String> {
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

    fn create(&self, text: String) -> Result<(), String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO todos (text, completed) VALUES (?1, 0)",
            [&text],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn toggle(&self, id: u32) -> Result<(), String> {
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
        Ok(())
    }

    fn delete(&self, id: u32) -> Result<(), String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM todos WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
