use super::model::{UserPersona, UserPreferences};
use super::repository::RecommendationRepository;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct RecommendationState {
    pub repo: Arc<dyn RecommendationRepository>,
    pub persona: Mutex<UserPersona>,
    pub client: reqwest::Client,
}

impl RecommendationState {
    pub fn new(repo: Arc<dyn RecommendationRepository>) -> Self {
        Self {
            repo,
            persona: Mutex::new(UserPersona::default()),
            client: reqwest::Client::new(),
        }
    }

    pub fn save_persona(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let persona = self.persona.lock().unwrap();
        let persona_path = app_dir.join("user_persona.json");
        let _ = fs::write(
            persona_path,
            serde_json::to_string(&*persona).unwrap_or_default(),
        );
    }

    pub fn load_persona(&self, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        let persona_path = app_dir.join("user_persona.json");
        if persona_path.exists() {
            if let Ok(content) = fs::read_to_string(persona_path) {
                if let Ok(saved) = serde_json::from_str::<UserPersona>(&content) {
                    *self.persona.lock().unwrap() = saved;
                }
            }
        }
    }

    pub fn save_preferences(&self, prefs: UserPreferences, app_handle: &tauri::AppHandle) {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        if !app_dir.exists() {
            let _ = fs::create_dir_all(&app_dir);
        }
        let prefs_path = app_dir.join("user_preferences.json");
        let _ = fs::write(
            prefs_path,
            serde_json::to_string(&prefs).unwrap_or_default(),
        );
    }

    pub fn load_preferences(&self, app_handle: &tauri::AppHandle) -> UserPreferences {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from("."));
        let prefs_path = app_dir.join("user_preferences.json");
        if prefs_path.exists() {
            if let Ok(content) = fs::read_to_string(prefs_path) {
                if let Ok(saved) = serde_json::from_str::<UserPreferences>(&content) {
                    return saved;
                }
            }
        }
        UserPreferences::default()
    }
}
