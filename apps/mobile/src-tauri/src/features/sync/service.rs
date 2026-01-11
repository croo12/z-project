use crate::features::recommendation::model::Article;
use crate::features::sync::client::{BrainServerClient, CreateArticleRequest};
use reqwest::Client;

/// Service for syncing articles with the Brain Server
pub struct SyncService {
    client: BrainServerClient,
}

impl SyncService {
    pub fn new(http_client: Client) -> Self {
        Self {
            client: BrainServerClient::new(http_client),
        }
    }

    /// Sync an article to the Brain Server
    /// Returns the server-side article ID if successful
    pub async fn sync_article(&self, article: &Article) -> Result<String, String> {
        // Convert tags to strings
        let tags: Vec<String> = article.tags.iter().map(|t| t.to_string()).collect();

        let request = CreateArticleRequest {
            title: article.title.clone(),
            url: article.url.clone(),
            content: article.summary.clone(), // Use summary as content
            tags,
        };

        let response = self.client.create_article(request).await?;
        Ok(response.id)
    }

    /// Submit feedback for an article to the Brain Server
    pub async fn submit_feedback(
        &self,
        server_article_id: &str,
        helpful: bool,
        reason: Option<&str>,
    ) -> Result<(), String> {
        self.client
            .submit_feedback(server_article_id, helpful, reason)
            .await
    }

    /// Check if the Brain Server is healthy
    pub async fn health_check(&self) -> bool {
        self.client.health_check().await
    }
}
