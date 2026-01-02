use super::model::{Article, ArticleCategory};

pub struct Scorer;

impl Scorer {
    /// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
    /// Positive score: Keep/Promote. Negative score: Demote/Discard.
    pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
        let mut score = 0;
        let title_lower = article.title.to_lowercase();
        let summary_lower = article.summary.to_lowercase();
        let content_to_check = format!("{} {}", title_lower, summary_lower);

        // High Impact Keywords
        let high_impact = [
            "rust",
            "tauri",
            "react",
            "typescript",
            "javascript",
            "android",
            "kotlin",
            "webassembly",
            "wasm",
            "docker",
            "kubernetes",
            "llvm",
            "compiler",
        ];
        // Medium Impact Keywords
        let medium_impact = [
            "code",
            "programming",
            "developer",
            "api",
            "frontend",
            "backend",
            "database",
            "algorithm",
            "git",
            "linux",
            "windows",
            "macos",
            "design pattern",
            "refactoring",
        ];
        // Negative Keywords (Noise Filter)
        let negative = [
            "stock",
            "market",
            "buffett",
            "berkshire",
            "invest",
            "politics",
            "crime",
            "murder",
            "sport",
            "celebrity",
            "gossip",
            "bitcoin",
            "crypto",
            "blockchain",
        ];

        for word in high_impact.iter() {
            if content_to_check.contains(word) {
                score += 10;
            }
        }
        for word in medium_impact.iter() {
            if content_to_check.contains(word) {
                score += 3;
            }
        }
        for word in negative.iter() {
            if content_to_check.contains(word) {
                score -= 20; // Strong penalty
            }
        }

        // Category Bonus using Tags
        for tag in &article.tags {
            // 1. Explicit User Interest Bonus (Primary Filter)
            if user_interests.contains(tag) {
                score += 50; // Huge boost for explicit selection
            }

            // 2. General Tech Bonus
            match tag {
                ArticleCategory::Rust
                | ArticleCategory::Tauri
                | ArticleCategory::React
                | ArticleCategory::Android => {
                    score += 5;
                }
                ArticleCategory::General => {
                    // No bonus
                }
                _ => {
                    score += 2;
                }
            }
        }

        // Feedback Logic (User Override)
        // If feedback exists (Positive or Negative), consider it "Read/Processed" and remove from recommendations.
        if article.feedback.is_some() {
            score -= 1000;
        }

        score
    }
}
