pub mod client;
pub mod service;

use crate::error::AppError;
use crate::features::recommendation::model::Article;
use crate::features::recommendation::system::RecommendationState;
use service::SyncService;
use tauri::State;

/// Sync an article to the Brain Server
#[tauri::command]
pub async fn sync_article_to_server(
    article: Article,
    state: State<'_, RecommendationState>,
) -> Result<(), AppError> {
    let sync_service = SyncService::new(state.client.clone());

    // Check if already synced
    if let Ok(true) = state.repo.is_article_synced(&article.id) {
        return Ok(());
    }

    // Sync to server
    match sync_service.sync_article(&article).await {
        Ok(server_article_id) => {
            // Mark as synced in local DB
            let _ = state
                .repo
                .mark_article_synced(&article.id, &server_article_id);
            Ok(())
        }
        Err(e) => {
            // Log error but don't fail - server sync is optional
            eprintln!("Failed to sync article to server: {}", e);
            Ok(())
        }
    }
}

/// Check if the Brain Server is healthy
#[tauri::command]
pub async fn check_server_health(state: State<'_, RecommendationState>) -> Result<bool, AppError> {
    let sync_service = SyncService::new(state.client.clone());
    Ok(sync_service.health_check().await)
}
