use crate::error::AppError;
use crate::features::recommendation::ai::AIService;
use crate::features::recommendation::config::FEEDS;
use crate::features::recommendation::model::{Article, ArticleCategory};
use crate::features::recommendation::service::{calculate_relevance_score, fetch_feed};
use crate::features::recommendation::system::RecommendationState;
use tauri::State;

#[tauri::command]
pub async fn fetch_articles(state: State<'_, RecommendationState>) -> Result<usize, AppError> {
    let mut all_fetched = Vec::new();

    // Optimization: Reuse client and fetch concurrently
    let mut handles = Vec::new();

    for (url, category) in FEEDS.iter() {
        let client = state.client.clone();
        let url = url.to_string();
        let category = category.clone();
        handles.push(tauri::async_runtime::spawn(async move {
            fetch_feed(&url, category, &client).await
        }));
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(fetched)) => {
                all_fetched.extend(fetched);
            }
            Ok(Err(e)) => {
                eprintln!("Error fetching feed: {}", e);
            }
            Err(e) => {
                eprintln!("Task failed to complete: {}", e);
            }
        }
    }

    // Deduplication & Merge Logic
    // Optimized: Use batch upsert in a single transaction to reduce database overhead.
    let new_count = state.repo.upsert_articles(all_fetched)?;

    Ok(new_count)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<Vec<Article>, AppError> {
    // DB Access
    let articles = state.repo.get_articles()?;

    // 1. Calculate Scores & Sort
    let prefs = state.load_preferences(&app);
    let mut scored_articles: Vec<(i32, Article)> = articles
        .into_iter()
        .map(|a| (calculate_relevance_score(&a, &prefs.interested_tags), a))
        .filter(|(score, _)| *score > -10)
        .collect();

    // Sort by Score DESC, then Date DESC
    scored_articles.sort_by(|(score_a, article_a), (score_b, article_b)| {
        score_b
            .cmp(score_a)
            .then_with(|| article_b.published_at.cmp(&article_a.published_at))
    });

    let top_candidates: Vec<Article> = scored_articles.into_iter().map(|(_, a)| a).collect();

    // 2. Rule-based: Top 3 (Highest Scored + Newest)
    let top_3: Vec<Article> = top_candidates.iter().take(3).cloned().collect();
    let remaining: Vec<Article> = top_candidates.iter().skip(3).cloned().collect();

    // 3. AI-based: Next 4 from remaining
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();

    let ai_picks = if !api_key.is_empty() && !remaining.is_empty() {
        let candidates_for_ai: Vec<Article> = remaining.into_iter().take(20).collect();
        let persona = state.persona.lock().unwrap().clone();

        AIService::recommend_with_gemini(
            candidates_for_ai,
            &persona,
            &prefs.interested_tags,
            api_key,
            &state.client,
        )
        .await
    } else {
        remaining.into_iter().take(4).collect()
    };

    // 4. Combine
    let mut result = top_3;
    result.extend(ai_picks);

    Ok(result)
}

#[tauri::command]
pub async fn submit_feedback(
    id: String,
    helpful: bool,
    reason: String,
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    let timestamp = chrono::Local::now().to_rfc3339();
    state
        .repo
        .update_feedback(&id, helpful, &reason, &timestamp)?;

    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if !api_key.is_empty() {
        let count = state.repo.get_feedback_count()?;

        if count > 0 && count % 3 == 0 {
            println!("Triggering Persona Update (Feedback Count: {})", count);
            let all_feedback = state.repo.get_feedback()?;
            let current_persona = state.persona.lock().unwrap().clone();

            let persona_update_result = AIService::update_user_persona(
                &all_feedback,
                &current_persona,
                &api_key,
                &state.client,
            )
            .await;

            if let Ok(new_persona) = persona_update_result {
                *state.persona.lock().unwrap() = new_persona;
                state.save_persona(&app);
            } else if let Err(e) = persona_update_result {
                eprintln!("Failed to update user persona: {}", e);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn save_user_interests(
    categories: Vec<ArticleCategory>,
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<(), AppError> {
    let mut prefs = state.load_preferences(&app);
    prefs.interested_tags = categories;
    state.save_preferences(prefs, &app);
    Ok(())
}

#[tauri::command]
pub async fn get_user_interests(
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<Vec<ArticleCategory>, AppError> {
    let prefs = state.load_preferences(&app);
    Ok(prefs.interested_tags)
}
