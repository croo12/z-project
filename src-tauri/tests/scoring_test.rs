use tauri_app_lib::features::recommendation::model::{Article, ArticleCategory, Feedback};
use tauri_app_lib::features::recommendation::service::calculate_relevance_score;

#[test]
fn test_scoring_integration() {
    // 1. Explicit Filtering Test
    let rust_article = Article {
        id: "1".into(),
        title: "Rust Updates".into(),
        summary: "New async features".into(),
        url: "http://rust.com".into(),
        tags: vec![ArticleCategory::Rust],
        published_at: "".into(),
        feedback: None,
        image_url: None,
        author: None,
    };
    
    // User likes Rust
    let score_rust = calculate_relevance_score(&rust_article, &[ArticleCategory::Rust]);
    let score_none = calculate_relevance_score(&rust_article, &[]);

    assert!(score_rust > score_none, "Explicit interest should boost score");
    assert!(score_rust >= 60, "Score should be at least 50+10 ({})", score_rust);

    // 2. Feedback Logic Test (The User's specific complaint)
    let downvoted_article = Article {
        id: "2".into(),
        title: "Bad Article".into(),
        summary: "Spam".into(),
        url: "http://spam.com".into(),
        tags: vec![ArticleCategory::General],
        published_at: "".into(),
        feedback: Some(Feedback {
            is_helpful: false,
            reason: "Spam".into(),
            created_at: "".into(),
        }),
        image_url: None,
        author: None,
    };

    let upvoted_article = Article {
        id: "3".into(),
        title: "Good Article".into(),
        summary: "Nice".into(),
        url: "http://nice.com".into(),
        tags: vec![ArticleCategory::General],
        published_at: "".into(),
        feedback: Some(Feedback {
            is_helpful: true,
            reason: "Nice".into(),
            created_at: "".into(),
        }),
        image_url: None,
        author: None,
    };

    let score_bad = calculate_relevance_score(&downvoted_article, &[]);
    let score_read = calculate_relevance_score(&upvoted_article, &[]);

    // Both should be filtered out
    assert!(score_bad < -500, "Downvoted article should be buried (Score: {})", score_bad);
    assert!(score_read < -500, "Upvoted article should be hidden as read (Score: {})", score_read);
}
