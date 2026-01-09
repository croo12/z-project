#![cfg(test)]

use crate::features::recommendation::{
    model::{Article, ArticleCategory},
    scoring::calculate_relevance_score,
};

#[test]
fn test_scoring_system() {
    // Case 1: High Relevance (Rust)
    let rust_article = Article {
        id: "1".into(),
        title: "Rust 1.85 Released with Async improvements".into(),
        summary: "Great new features for memory safety and performance.".into(),
        url: "".into(),
        tags: vec![ArticleCategory::Rust],
        published_at: "".into(),
        feedback: None,
        image_url: None,
        author: None,
    };

    // Case 2: Noise (Finance/Stock)
    let stock_article = Article {
        id: "2".into(),
        title: "Warren Buffett sells Berkshire Hathaway stock".into(),
        summary: "Market analysis of the recent finance trends.".into(),
        url: "".into(),
        tags: vec![ArticleCategory::General],
        published_at: "".into(),
        feedback: None,
        image_url: None,
        author: None,
    };

    // Case 3: Mixed (General Tech)
    let tech_article = Article {
        id: "3".into(),
        title: "10 Tips for cleaner Code".into(),
        summary: "Refactoring tips for developers.".into(),
        url: "".into(),
        tags: vec![ArticleCategory::General],
        published_at: "".into(),
        feedback: None,
        image_url: None,
        author: None,
    };

    let s1 = calculate_relevance_score(&rust_article, &[]);
    let s2 = calculate_relevance_score(&stock_article, &[]);
    let s3 = calculate_relevance_score(&tech_article, &[]);

    println!("Rust Score: {}", s1);
    println!("Stock Score: {}", s2);
    println!("Tech Score: {}", s3);

    assert!(
        s1 > s3,
        "Rust specific should score higher than general code"
    );
    assert!(
        s3 > s2,
        "General code should score higher than stock market noise"
    );
    assert!(s2 < 0, "Stock market noise should have negative score");
}
