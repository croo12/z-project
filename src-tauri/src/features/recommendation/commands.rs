use crate::error::AppError;
use crate::features::recommendation::model::{Article, ArticleCategory};
use crate::features::recommendation::service::{
    calculate_relevance_score, fetch_feed, recommend_with_gemini, update_user_persona,
};
use crate::features::recommendation::system::RecommendationState;
use tauri::State;

#[tauri::command]
pub async fn fetch_articles(state: State<'_, RecommendationState>) -> Result<usize, AppError> {
    let feeds = vec![
        // Rust
        ("https://blog.rust-lang.org/feed.xml", ArticleCategory::Rust),
        (
            "https://this-week-in-rust.org/rss.xml",
            ArticleCategory::Rust,
        ),
        // Android / Kotlin
        (
            "https://feeds.feedburner.com/blogspot/hsDu",
            ArticleCategory::Android,
        ),
        ("https://androidweekly.net/rss", ArticleCategory::Android),
        // Tauri
        ("https://tauri.app/blog/rss.xml", ArticleCategory::Tauri),
        // Web / TypeScript
        (
            "https://devblogs.microsoft.com/typescript/feed/",
            ArticleCategory::TypeScript,
        ),
        ("https://css-tricks.com/feed/", ArticleCategory::Web),
        (
            "https://www.smashingmagazine.com/feed/",
            ArticleCategory::Web,
        ),
        ("https://web.dev/feed.xml", ArticleCategory::Web),
        ("https://fettblog.eu/feed.xml", ArticleCategory::TypeScript),
        (
            "https://levelup.gitconnected.com/feed",
            ArticleCategory::Web,
        ),
        (
            "https://2ality.com/feeds/posts.xml",
            ArticleCategory::TypeScript,
        ),
        // React
        ("https://react.dev/feed.xml", ArticleCategory::React),
        ("https://overreacted.io/rss.xml", ArticleCategory::React),
        ("https://tkdodo.eu/blog/rss.xml", ArticleCategory::React),
        (
            "https://kentcdodds.com/blog/rss.xml",
            ArticleCategory::React,
        ),
        (
            "https://www.joshwcomeau.com/rss.xml",
            ArticleCategory::React,
        ),
        ("https://robinwieruch.de/index.xml", ArticleCategory::React),
        ("https://ui.dev/blog/rss", ArticleCategory::React),
        (
            "https://www.developerway.com/rss.xml",
            ArticleCategory::React,
        ),
        // AI
        ("https://openai.com/blog/rss.xml", ArticleCategory::AI),
        ("https://blogs.microsoft.com/ai/feed/", ArticleCategory::AI),
        // General / Tech
        ("https://news.ycombinator.com/rss", ArticleCategory::General),
        ("https://dev.to/feed", ArticleCategory::General),
    ];

    let mut new_count = 0;
    let mut all_fetched = Vec::new();

    // Optimization: Reuse client and fetch concurrently
    let mut handles = Vec::new();

    for (url, category) in feeds {
        let client = state.client.clone();
        let url = url.to_string();
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
    // We can't batch insert easily with SQLite due to unique constraints and logic needed per item,
    // so we iterate.
    for mut item in all_fetched {
        // Check if exists
        let existing_tags_json = state.repo.check_article_exists(&item.url)?;

        let final_tags = if let Some(tags_str) = existing_tags_json {
            // Merge
            let mut current_tags: Vec<ArticleCategory> = serde_json::from_str(&tags_str)?;
            for new_tag in item.tags {
                if !current_tags.contains(&new_tag) {
                    current_tags.push(new_tag);
                }
            }
            current_tags
        } else {
            // New
            new_count += 1;
            item.tags
        };

        // We need to update the item with merged tags before saving
        item.tags = final_tags;
        state.repo.save_article(item)?;
    }

    Ok(new_count)
}

#[tauri::command]
pub async fn get_recommended_articles(
    state: State<'_, RecommendationState>,
    app: tauri::AppHandle,
) -> Result<Vec<Article>, AppError> {
    // DB Access
    let articles = state.repo.get_candidate_articles()?;

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

        recommend_with_gemini(
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

            let persona_update_result =
                update_user_persona(&all_feedback, &current_persona, &api_key, &state.client).await;

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
