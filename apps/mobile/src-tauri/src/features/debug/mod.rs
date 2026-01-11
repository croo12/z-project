#[cfg(debug_assertions)]
pub mod commands;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("debug")
        .invoke_handler(tauri::generate_handler![
            #[cfg(debug_assertions)]
            commands::test_ai_connection
        ])
        .build()
}
