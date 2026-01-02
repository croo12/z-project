use super::model::WorkLog;
use super::service::WorkLogState;
use tauri::State;

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
