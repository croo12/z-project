use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_SERVER_URL: &str = "http://localhost:3000";
const REQUEST_TIMEOUT_SECS: u64 = 5;

#[derive(Debug, Serialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub url: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleResponse {
    pub id: String,
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "chunkCount")]
    pub chunk_count: i32,
    pub rating: f64,
    #[serde(rename = "positiveCount")]
    pub positive_count: i32,
    #[serde(rename = "negativeCount")]
    pub negative_count: i32,
}

#[derive(Debug, Serialize)]
pub struct SubmitFeedbackRequest {
    #[serde(rename = "type")]
    pub feedback_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Brain Server API Client
pub struct BrainServerClient {
    client: Client,
    base_url: String,
}

impl BrainServerClient {
    pub fn new(client: Client) -> Self {
        let base_url =
            std::env::var("BRAIN_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string());

        Self { client, base_url }
    }

    /// Create an article on the Brain Server
    pub async fn create_article(
        &self,
        request: CreateArticleRequest,
    ) -> Result<CreateArticleResponse, String> {
        let url = format!("{}/articles", self.base_url);

        let response = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Server returned error {}: {}", status, body));
        }

        response
            .json::<CreateArticleResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Submit feedback for an article on the Brain Server
    pub async fn submit_feedback(
        &self,
        server_article_id: &str,
        helpful: bool,
        reason: Option<&str>,
    ) -> Result<(), String> {
        let url = format!("{}/articles/{}/feedback", self.base_url, server_article_id);

        let request = SubmitFeedbackRequest {
            feedback_type: if helpful {
                "positive".to_string()
            } else {
                "negative".to_string()
            },
            comment: reason.map(|s| s.to_string()),
        };

        let response = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send feedback: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Server returned error {}: {}", status, body));
        }

        Ok(())
    }

    /// Check if the Brain Server is healthy
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/", self.base_url);

        match self
            .client
            .get(&url)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}
