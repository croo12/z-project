use super::model::WorkLog;
use super::repository::WorkLogRepository;
use std::sync::Arc;

pub struct WorkLogState {
    repo: Arc<dyn WorkLogRepository + Send + Sync>,
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
