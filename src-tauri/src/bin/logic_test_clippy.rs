// Test file to run clippy on logic only
#[path = "../features/recommendation/config.rs"]
mod config;

#[path = "../features/recommendation/model.rs"]
mod model;

// We need to pull in service.rs but it has reqwest which is heavy.
// Ideally we just want to test calculate_relevance_score which is pure logic.
// But the unit tests are inside service.rs.

// Strategy: Copy the calculate_relevance_score and the test to here to verify logic.
// Real verification happens when 'cargo test' runs on the real file.
// But I can't run 'cargo test' easily due to glib.
// So I will replicate the test here.

use config::{HIGH_IMPACT_KEYWORDS, MEDIUM_IMPACT_KEYWORDS, NEGATIVE_KEYWORDS};
use model::{Article, ArticleCategory, Feedback};

pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
    let mut score = 0;
    let title_lower = article.title.to_lowercase();
    let summary_lower = article.summary.to_lowercase();

    for word in HIGH_IMPACT_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
            score += 10;
        }
    }
    for word in MEDIUM_IMPACT_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
            score += 3;
        }
    }
    for word in NEGATIVE_KEYWORDS.iter() {
        if title_lower.contains(word) || summary_lower.contains(word) {
            score -= 20;
        }
    }

    for tag in &article.tags {
        if user_interests.contains(tag) {
            score += 50;
        }
        match tag {
            ArticleCategory::Rust
            | ArticleCategory::Tauri
            | ArticleCategory::React
            | ArticleCategory::Android => {
                score += 5;
            }
            ArticleCategory::General => {
            }
            _ => {
                score += 2;
            }
        }
    }

    if article.feedback.is_some() {
        score -= 1000;
    }

    score
}

fn main() {
        // Case: Downvoted article
        let downvoted_article = Article {
            id: "down".into(),
            title: "Bad Article".into(),
            summary: "Not helpful".into(),
            url: "http://bad.com".into(),
            tags: vec![ArticleCategory::Rust],
            published_at: "".into(),
            feedback: Some(Feedback {
                is_helpful: false,
                reason: "Bad".into(),
                created_at: "".into(),
            }),
            image_url: None,
            author: None,
        };

        // Case: Upvoted (Already Read) article
        let upvoted_article = Article {
            id: "up".into(),
            title: "Good Article".into(),
            summary: "Helpful".into(),
            url: "http://good.com".into(),
            tags: vec![ArticleCategory::Rust],
            published_at: "".into(),
            feedback: Some(Feedback {
                is_helpful: true,
                reason: "Good".into(),
                created_at: "".into(),
            }),
            image_url: None,
            author: None,
        };

        let s1 = calculate_relevance_score(&downvoted_article, &[]);
        let s2 = calculate_relevance_score(&upvoted_article, &[]);

        assert!(
            s1 < -500,
            "Downvoted article should be buried (-1000 penalty)"
        );
        assert!(
            s2 < -500,
            "Upvoted article should also be hidden (treated as read)"
        );
        println!("Tests Passed!");
}
