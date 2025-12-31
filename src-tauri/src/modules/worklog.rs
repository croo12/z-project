use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkLog {
    pub id: u32,
    pub project: String,
    pub hours: f32,
    pub date: String,
}

pub struct WorkLogState {
    pub logs: Mutex<Vec<WorkLog>>,
}

impl Default for WorkLogState {
    fn default() -> Self {
        Self {
            logs: Mutex::new(Vec::new()),
        }
    }
}

impl WorkLogState {
    pub fn with_demo_data() -> Self {
        Self {
            logs: Mutex::new(vec![WorkLog {
                id: 1,
                project: "Personal App".to_string(),
                hours: 2.5,
                date: "2025-12-29".to_string(),
            }]),
        }
    }

    pub fn add(&self, project: String, hours: f32) -> Vec<WorkLog> {
        let mut logs = self.logs.lock().unwrap();
        let id = logs.len() as u32 + 1;
        let date = "2025-12-29".to_string(); // In a real app, use chrono::Local::now()
        logs.push(WorkLog {
            id,
            project,
            hours,
            date,
        });
        logs.clone()
    }

    pub fn get_all(&self) -> Vec<WorkLog> {
        self.logs.lock().unwrap().clone()
    }
}

// --- Commands ---

#[tauri::command]
pub fn get_work_logs(state: State<WorkLogState>) -> Vec<WorkLog> {
    state.get_all()
}

#[tauri::command]
pub fn add_work_log(project: String, hours: f32, state: State<WorkLogState>) -> Vec<WorkLog> {
    state.add(project, hours)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_work_log() {
        let state = WorkLogState::default();
        let logs = state.add("Tauri App".to_string(), 4.5);
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].project, "Tauri App");
        assert_eq!(logs[0].hours, 4.5);
    }

    #[test]
    fn test_get_work_logs() {
        let state = WorkLogState::with_demo_data();
        let logs = state.get_all();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].project, "Personal App");
    }
}
