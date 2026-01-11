use crate::error::AppError;
use crate::features::recommendation::system::RecommendationState;
use tauri::State;

#[tauri::command]
pub async fn test_ai_connection(
    _state: State<'_, RecommendationState>,
) -> Result<String, AppError> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();

    if api_key.is_empty() {
        return Ok("❌ No GEMINI_API_KEY found in environment variables".to_string());
    }

    let masked = if api_key.len() > 8 {
        format!("{}...{}", &api_key[0..4], &api_key[api_key.len() - 4..])
    } else {
        "***".to_string()
    };

    Ok(format!("✅ GEMINI_API_KEY detected: {}", masked))
}
