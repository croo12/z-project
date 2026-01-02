use super::model::WorkLog;
use super::repository::WorkLogRepository;
use std::sync::Arc;
use tauri::State;

pub struct WorkLogState {
    pub repo: Arc<dyn WorkLogRepository + Send + Sync>,
}

impl WorkLogState {
    pub fn new(repo: Arc<dyn WorkLogRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub fn add(&self, project: String, hours: f32) -> Result<Vec<WorkLog>, String> {
        self.repo.create(project, hours)?;
        self.get_all()
    }

    pub fn get_all(&self) -> Result<Vec<WorkLog>, String> {
        self.repo.get_all()
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
