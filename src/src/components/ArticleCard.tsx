import { memo } from "react";
import type { Article, ArticleCategory } from "../types";
import FeedbackForm from "./FeedbackForm";

interface ArticleCardProps {
  article: Article;
  isFeedbacking: boolean;
  onSetFeedbackingId: (id: string | null) => void;
  onSubmitFeedback: (id: string, helpful: boolean, reason: string) => void;
}

function getCategoryColor(cat: ArticleCategory): string {
  switch (cat) {
    case "Rust":
      return "#dea584";
    case "React":
      return "#61dafb";
    case "Android":
      return "#3ddc84";
    case "Tauri":
      return "#ffc131";
    case "TypeScript":
      return "#3178c6";
    default:
      return "#888";
  }
}

const ArticleCard = memo(function ArticleCard({
  article,
  isFeedbacking,
  onSetFeedbackingId,
  onSubmitFeedback,
}: ArticleCardProps) {
  const primaryTag = article.tags[0] || "General";
  return (
    <div
      className="card article-item"
      style={{ borderLeft: `4px solid ${getCategoryColor(primaryTag)}` }}
    >
      <div
        style={{
          display: "flex",
          gap: "4px",
          flexWrap: "wrap",
          marginBottom: "8px",
        }}
      >
        {article.tags.map((tag) => (
          <span
            key={tag}
            style={{
              fontSize: "0.7rem",
              fontWeight: "bold",
              color: "#fff",
              backgroundColor: getCategoryColor(tag),
              padding: "2px 6px",
              borderRadius: "4px",
            }}
          >
            {tag}
          </span>
        ))}
        <span style={{ fontSize: "0.8rem", color: "#888", marginLeft: "auto" }}>
          {new Date(article.published_at).toLocaleDateString()}
        </span>
      </div>
      <h3>{article.title}</h3>
      {article.image_url && (
        <img
          src={article.image_url}
          alt={article.title}
          loading="lazy"
          style={{
            width: "100%",
            height: "150px",
            objectFit: "cover",
            borderRadius: "4px",
            marginBottom: "0.5rem",
          }}
        />
      )}
      <p>{article.summary}</p>
      {article.author && (
        <small
          style={{ display: "block", color: "#666", marginBottom: "0.5rem" }}
        >
          By {article.author}
        </small>
      )}
      <a href={article.url} target="_blank" className="news-link" rel="noreferrer">
        Read Article &rarr;
      </a>

      {/* Feedback UI */}
      <div
        className="feedback-section"
        style={{
          marginTop: "1rem",
          borderTop: "1px solid #eee",
          paddingTop: "0.5rem",
        }}
      >
        {article.feedback ? (
          <small>
            Feedback:{" "}
            {article.feedback.is_helpful ? "👍 Helpful" : "👎 Not Helpful"}(
            {article.feedback.reason})
          </small>
        ) : isFeedbacking ? (
          <FeedbackForm
            onSubmit={(helpful, reason) =>
              onSubmitFeedback(article.id, helpful, reason)
            }
            onCancel={() => onSetFeedbackingId(null)}
          />
        ) : (
          <button
            onClick={() => onSetFeedbackingId(article.id)}
            style={{
              fontSize: "0.8rem",
              padding: "4px 8px",
              background: "transparent",
              color: "#888",
              border: "1px solid #eee",
            }}
          >
            Rate this article
          </button>
        )}
      </div>
    </div>
  );
});

export default ArticleCard;
