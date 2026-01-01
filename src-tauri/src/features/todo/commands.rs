use super::model::Todo;
use super::service::TodoState;
use tauri::State;

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
