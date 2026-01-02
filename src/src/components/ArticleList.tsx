import { useState, useMemo, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ArticleCategory } from "../types";
import type { Article } from "../types";
import "../App.css";
import ArticleCard from "./ArticleCard";

const CATEGORIES: ArticleCategory[] = Object.values(ArticleCategory);

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
      : articles.filter((a) => a.tags.includes(filter));
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
      await invoke("submit_feedback", { id, helpful, reason });
      setFeedbackingId(null);
      onFeedbackUpdate();
    },
    [onFeedbackUpdate]
  );

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
            onSetFeedbackingId={setFeedbackingId}
            onSubmitFeedback={handleSubmitFeedback}
          />
        ))}
        {filtered.length === 0 && <p>No articles found.</p>}
      </div>
    </div>
  );
}
