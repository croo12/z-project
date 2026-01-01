use crate::db::DbPool;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkLog {
    pub id: u32,
    pub project: String,
    pub hours: f32,
    pub date: String,
}

pub struct WorkLogState {
    pub pool: DbPool,
}

impl WorkLogState {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn add(&self, project: String, hours: f32) -> Result<Vec<WorkLog>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Cast f32 to f64 for SQLite REAL compatibility
        let hours_f64 = hours as f64;

        conn.execute(
            "INSERT INTO work_logs (project, hours, date) VALUES (?1, ?2, ?3)",
            rusqlite::params![project, hours_f64, date],
        )
        .map_err(|e| e.to_string())?;

        self.get_all()
    }

    pub fn get_all(&self) -> Result<Vec<WorkLog>, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, project, hours, date FROM work_logs ORDER BY id DESC")
            .map_err(|e| e.to_string())?;

        let log_iter = stmt
            .query_map([], |row| {
                let hours_f64: f64 = row.get(2)?;
                Ok(WorkLog {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    hours: hours_f64 as f32,
                    date: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut logs = Vec::new();
        for log in log_iter {
            logs.push(log.map_err(|e| e.to_string())?);
        }
        Ok(logs)
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_work_logs(state: State<WorkLogState>) -> Result<Vec<WorkLog>, String> {
    state.get_all()
}

#[tauri::command]
pub fn add_work_log(
    project: String,
    hours: f32,
    state: State<WorkLogState>,
) -> Result<Vec<WorkLog>, String> {
    state.add(project, hours)
}
