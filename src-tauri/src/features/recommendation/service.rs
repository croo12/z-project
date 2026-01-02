use super::model::{Article, ArticleCategory, Feedback, UserPersona};
use super::parser::RssParser;
use super::scorer::Scorer;
use reqwest;

pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
    Scorer::calculate_relevance_score(article, user_interests)
}

pub async fn fetch_feed(
    url: &str,
    source_category: ArticleCategory,
    client: &reqwest::Client,
) -> Result<Vec<Article>, String> {
    let content = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let parser = RssParser::new();
    parser.parse(&content, source_category)
}

pub async fn update_user_persona(
    feedback_history: &[Feedback],
    current_persona: &UserPersona,
    api_key: &str,
) -> Result<UserPersona, String> {
    if feedback_history.is_empty() {
        return Ok(current_persona.clone());
    }

    let mut prompt = String::from("You are an expert user analyst. Update the User Persona based on the recent feedback provided.\n\n");

    if !current_persona.description.is_empty() {
        prompt.push_str(&format!(
            "CURRENT PERSONA:\n{}\n\n",
            current_persona.description
        ));
    }

    prompt.push_str("RECENT FEEDBACK:\n");
    for f in feedback_history.iter().take(20) {
        // Analyze last 20 feedback items
        prompt.push_str(&format!(
            "- Helpful: {}, Reason: {}\n",
            f.is_helpful, f.reason
        ));
    }

    prompt.push_str("\nTask: Analyze the feedback patterns to refine the User Persona.\n");
    prompt.push_str("INSTRUCTIONS:\n");
    prompt.push_str(
        "1. Identify specific keywords or topics the user explicitly LIKES (Helpful=true).\n",
    );
    prompt.push_str("2. Identify topics the user DISLIKES (Helpful=false).\n");
    prompt.push_str("3. Update the description to be specific (e.g., 'User prefers Rust async and Tauri architecture, but dislikes general finance news').\n");
    prompt.push_str("4. Output ONLY the concise description text (2-3 sentences).");

    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}", api_key))
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if let Ok(json) = res.json::<serde_json::Value>().await {
        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            return Ok(UserPersona {
                description: text.trim().to_string(),
                last_updated: chrono::Local::now().to_rfc3339(),
            });
        }
    }

    Err("Failed to generate persona".to_string())
}

pub async fn recommend_with_gemini(
    candidates: Vec<Article>,
    persona: &UserPersona,
    user_interests: &[ArticleCategory],
    api_key: String,
) -> Vec<Article> {
    // 1. Construct Prompt
    let mut prompt = String::from("You are a tech article recommender. Select the best 4 articles from the CANDIDATES list.\n\n");

    // Explicit Inputs
    if !user_interests.is_empty() {
        prompt.push_str(&format!("USER SELECTED TAGS: {:?}\n", user_interests));
        prompt.push_str("INSTRUCTION: Prioritize articles that match the USER SELECTED TAGS above all else.\n\n");
    }

    if !persona.description.is_empty() {
        prompt.push_str(&format!("USER PERSONA (Implicit Preferences):\n{}\n\nThen, refine the selection to match this persona.\n\n", persona.description));
    } else {
        prompt.push_str("Prioritize technical depth and relevance to Rust, Tauri, React, and System Programming.\n\n");
    }

    prompt.push_str("CANDIDATES (JSON):\n");
    let simple_candidates: Vec<_> = candidates
        .iter()
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "title": a.title,
                "tags": format!("{:?}", a.tags),
                "summary": a.summary.chars().take(150).collect::<String>()
            })
        })
        .collect();
    prompt.push_str(&serde_json::to_string(&simple_candidates).unwrap_or_default());

    prompt.push_str("\n\nRespond ONLY with a JSON array of the IDs of the 4 selected articles.");

    // 2. Call Gemini API
    let client = reqwest::Client::new();
    let res = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}", api_key))
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()
        .await;

    // 3. Parse Response
    if let Ok(response) = res {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                let clean_text = text
                    .replace("```json", "")
                    .replace("```", "")
                    .trim()
                    .to_string();
                if let Ok(selected_ids) = serde_json::from_str::<Vec<String>>(&clean_text) {
                    return candidates
                        .into_iter()
                        .filter(|a| selected_ids.contains(&a.id))
                        .collect();
                }
            }
        }
    }

    // Fallback or if AI fails
    candidates.into_iter().take(4).collect()
}
