use crate::features::recommendation::model::Article;
use crate::features::recommendation::service::calculate_relevance_score;
use std::time::Instant;

#[test]
fn bench_calculate_relevance_score() {
    let article = Article {
        id: "1".to_string(),
        title: "Rust is amazing for Tauri apps".to_string(),
        summary: "This article discusses how Rust and React work together in Tauri. It mentions webassembly and performance.".to_string(),
        url: "http://example.com".to_string(),
        tags: vec![],
        published_at: "".to_string(),
        feedback: None,
        image_url: None,
        author: None,
    };

    let start = Instant::now();
    for _ in 0..10000 {
        calculate_relevance_score(&article, &[]);
    }
    println!("Time taken: {:?}", start.elapsed());
}
