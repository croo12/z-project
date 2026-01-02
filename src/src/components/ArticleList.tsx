import { useState, useMemo } from "react";
import { ArticleCategory } from "../types";
import type { Article } from "../types";
import "../App.css";
import ArticleCard from "./ArticleCard";

const CATEGORIES: ArticleCategory[] = Object.values(ArticleCategory);

interface Props {
  articles: Article[];
  onRefresh: () => void;
  loading: boolean;
  onSubmitFeedback: (id: string, helpful: boolean, reason: string) => void;
}

export default function ArticleList({
  articles,
  onRefresh,
  loading,
  onSubmitFeedback,
}: Props) {
  const [filter, setFilter] = useState<ArticleCategory | "All">("All");
  const [feedbackingId, setFeedbackingId] = useState<string | null>(null);

  const filtered = useMemo(() => {
    return filter === "All"
      ? articles
      : articles.filter((a) => a.tags.includes(filter));
  }, [articles, filter]);

  const handleFeedbackSubmit = (id: string, helpful: boolean, reason: string) => {
    onSubmitFeedback(id, helpful, reason);
    setFeedbackingId(null);
  };

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
        <button onClick={onRefresh} disabled={loading}>
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
            onSetFeedbackingId={setFeedbackingId}
            onSubmitFeedback={handleFeedbackSubmit}
          />
        ))}
        {filtered.length === 0 && <p>No articles found.</p>}
      </div>
    </div>
  );
}
