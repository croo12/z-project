use crate::repositories::work_log::WorkLogRepository;
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
    pub repository: Arc<dyn WorkLogRepository + Send + Sync>,
}

impl WorkLogState {
    pub fn new(repository: Arc<dyn WorkLogRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_work_logs(state: State<WorkLogState>) -> Result<Vec<WorkLog>, String> {
    state.repository.get_all()
}

#[tauri::command]
pub fn add_work_log(
    project: String,
    hours: f32,
    state: State<WorkLogState>,
) -> Result<Vec<WorkLog>, String> {
    state.repository.create(project, hours)
}
