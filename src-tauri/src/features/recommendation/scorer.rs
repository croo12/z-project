use super::model::{Article, ArticleCategory, Feedback};

// Scoring Weights
const SCORE_EXPLICIT_INTEREST: i32 = 50;
const SCORE_HIGH_IMPACT: i32 = 10;
const SCORE_MEDIUM_IMPACT: i32 = 3;
const SCORE_NEGATIVE: i32 = -20;
const SCORE_TAG_BONUS_HIGH: i32 = 5;
const SCORE_TAG_BONUS_LOW: i32 = 2;
const SCORE_READ_PENALTY: i32 = -1000;

// Keyword Lists
const HIGH_IMPACT_KEYWORDS: &[&str] = &[
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

const MEDIUM_IMPACT_KEYWORDS: &[&str] = &[
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

const NEGATIVE_KEYWORDS: &[&str] = &[
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

/// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
/// Positive score: Keep/Promote. Negative score: Demote/Discard.
pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
    // 1. Feedback Logic (User Override)
    // If feedback exists (Positive or Negative), consider it "Read/Processed" and remove from recommendations.
    if article.feedback.is_some() {
        return SCORE_READ_PENALTY;
    }

    let mut score = 0;
    let title_lower = article.title.to_lowercase();
    let summary_lower = article.summary.to_lowercase();
    let content_to_check = format!("{} {}", title_lower, summary_lower);

    // 2. Keyword Scoring
    for word in HIGH_IMPACT_KEYWORDS {
        if content_to_check.contains(word) {
            score += SCORE_HIGH_IMPACT;
        }
    }
    for word in MEDIUM_IMPACT_KEYWORDS {
        if content_to_check.contains(word) {
            score += SCORE_MEDIUM_IMPACT;
        }
    }
    for word in NEGATIVE_KEYWORDS {
        if content_to_check.contains(word) {
            score += SCORE_NEGATIVE; // Strong penalty
        }
    }

    // 3. Category Bonus using Tags
    for tag in &article.tags {
        // Explicit User Interest Bonus (Primary Filter)
        if user_interests.contains(tag) {
            score += SCORE_EXPLICIT_INTEREST; // Huge boost for explicit selection
        }

        // General Tech Bonus
        match tag {
            ArticleCategory::Rust
            | ArticleCategory::Tauri
            | ArticleCategory::React
            | ArticleCategory::Android => {
                score += SCORE_TAG_BONUS_HIGH;
            }
            ArticleCategory::General => {
                // No bonus
            }
            _ => {
                score += SCORE_TAG_BONUS_LOW;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::recommendation::model::{Article, ArticleCategory, Feedback};

    #[test]
    fn test_feedback_scoring_internal() {
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

        assert_eq!(
            s1, SCORE_READ_PENALTY,
            "Downvoted article should have READ_PENALTY"
        );
        assert_eq!(
            s2, SCORE_READ_PENALTY,
            "Upvoted article should have READ_PENALTY"
        );
    }
}
