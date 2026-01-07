use super::config::{HIGH_IMPACT_KEYWORDS, MEDIUM_IMPACT_KEYWORDS, NEGATIVE_KEYWORDS};
use super::model::{Article, ArticleCategory, Feedback};

/// Calculates a relevance score for an article to filter out noise (e.g., Finance, Politics).
/// Positive score: Keep/Promote. Negative score: Demote/Discard.
pub fn calculate_relevance_score(article: &Article, user_interests: &[ArticleCategory]) -> i32 {
    let mut score = 0;
    // Optimization: Avoid allocating a new concatenated string. Check title and summary individually.
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

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(
            s1 < -500,
            "Downvoted article should be buried (-1000 penalty)"
        );
        assert!(
            s2 < -500,
            "Upvoted article should also be hidden (treated as read)"
        );
    }
}
