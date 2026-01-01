use crate::repositories::worklog::WorkLogRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkLog {
    pub id: u32,
    pub project: String,
    pub hours: f32,
    pub date: String,
}

pub struct WorkLogState {
    pub repo: Arc<dyn WorkLogRepository + Send + Sync>,
}

impl WorkLogState {
    pub fn new(repo: Arc<dyn WorkLogRepository + Send + Sync>) -> Self {
        Self { repo }
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_work_logs(state: State<WorkLogState>) -> Result<Vec<WorkLog>, String> {
    state.repo.get_all()
}

#[tauri::command]
pub fn add_work_log(
    project: String,
    hours: f32,
    state: State<WorkLogState>,
) -> Result<Vec<WorkLog>, String> {
    state.repo.create(project, hours)?;
    state.repo.get_all()
}
