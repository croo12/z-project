import { useState, useMemo, memo, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Article, ArticleCategory } from "../types";
import "../App.css";

const CATEGORIES: ArticleCategory[] = [
  "React",
  "Rust",
  "Android",
  "Tauri",
  "TypeScript",
  "General",
  "AI",
  "Web",
];

interface Props {
  articles: Article[];
  onRefresh: () => void;
  onFeedbackUpdate: () => void;
}

export default function ArticleList({
  articles,
  onRefresh,
  onFeedbackUpdate,
}: Props) {
  const [filter, setFilter] = useState<ArticleCategory | "All">("All");
  const [loading, setLoading] = useState(false);
  const [feedbackingId, setFeedbackingId] = useState<string | null>(null);

  const filtered = useMemo(() => {
    return filter === "All"
      ? articles
      : articles.filter((a) => a.category === filter);
  }, [articles, filter]);

  const handleRefresh = useCallback(async () => {
    setLoading(true);
    try {
      await invoke("fetch_articles");
      onRefresh();
    } finally {
      setLoading(false);
    }
  }, [onRefresh]);

  const handleSubmitFeedback = useCallback(
    async (id: string, helpful: boolean, reason: string) => {
      if (!reason.trim()) {
        alert("Please provide a reason.");
        return;
      }
      await invoke("submit_feedback", { id, helpful, reason });
      setFeedbackingId(null);
      onFeedbackUpdate();
    },
    [onFeedbackUpdate]
  );

  const handleCancelFeedback = useCallback(() => {
    setFeedbackingId(null);
  }, []);

  return (
    <div className="article-list-container">
      <div
        className="controls"
        style={{
          display: "flex",
          gap: "8px",
          flexWrap: "wrap",
          marginBottom: "1rem",
        }}
      >
        <button onClick={handleRefresh} disabled={loading}>
          {loading ? "Refreshing..." : "Refresh RSS"}
        </button>
        <button
          onClick={() => setFilter("All")}
          className={filter === "All" ? "active" : ""}
        >
          All
        </button>
        {CATEGORIES.map((cat) => (
          <button
            key={cat}
            onClick={() => setFilter(cat)}
            className={filter === cat ? "active" : ""}
            style={{ opacity: filter === cat ? 1 : 0.7 }}
          >
            {cat}
          </button>
        ))}
      </div>

      <div className="articles-grid" style={{ display: "grid", gap: "1rem" }}>
        {filtered.map((article) => (
          <ArticleCard
            key={article.id}
            article={article}
            isFeedbacking={feedbackingId === article.id}
            onStartFeedback={setFeedbackingId}
            onCancelFeedback={handleCancelFeedback}
            onSubmitFeedback={handleSubmitFeedback}
          />
        ))}
        {filtered.length === 0 && <p>No articles found.</p>}
      </div>
    </div>
  );
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

interface FeedbackInputProps {
  onSubmit: (helpful: boolean, reason: string) => void;
  onCancel: () => void;
}

function FeedbackInput({ onSubmit, onCancel }: FeedbackInputProps) {
  const [reason, setReason] = useState("");

  return (
    <div style={{ display: "flex", gap: "0.5rem", marginTop: "0.5rem" }}>
      <input
        value={reason}
        onChange={(e) => setReason(e.target.value)}
        placeholder="Reason..."
        autoFocus
        aria-label="Feedback reason"
      />
      <button onClick={() => onSubmit(true, reason)}>👍</button>
      <button onClick={() => onSubmit(false, reason)}>👎</button>
      <button onClick={onCancel} style={{ background: "#888" }}>
        Cancel
      </button>
    </div>
  );
}

interface ArticleCardProps {
  article: Article;
  isFeedbacking: boolean;
  onStartFeedback: (id: string) => void;
  onCancelFeedback: () => void;
  onSubmitFeedback: (id: string, helpful: boolean, reason: string) => void;
}

const ArticleCard = memo(function ArticleCard({
  article,
  isFeedbacking,
  onStartFeedback,
  onCancelFeedback,
  onSubmitFeedback,
}: ArticleCardProps) {
  return (
    <div
      className="card article-item"
      style={{ borderLeft: `4px solid ${getCategoryColor(article.category)}` }}
    >
      <div style={{ display: "flex", justifyContent: "space-between" }}>
        <span
          style={{
            fontSize: "0.8rem",
            fontWeight: "bold",
            color: getCategoryColor(article.category),
          }}
        >
          {article.category}
        </span>
        <span style={{ fontSize: "0.8rem", color: "#888" }}>
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
      <a href={article.url} target="_blank" className="news-link">
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
          <FeedbackInput
            onSubmit={(helpful, reason) =>
              onSubmitFeedback(article.id, helpful, reason)
            }
            onCancel={onCancelFeedback}
          />
        ) : (
          <button
            onClick={() => onStartFeedback(article.id)}
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
