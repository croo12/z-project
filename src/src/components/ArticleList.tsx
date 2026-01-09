import { useState, useMemo, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ArticleCategory } from "../types";
import type { Article } from "../types";
import ArticleCard from "./ArticleCard";
import { cn } from "../lib/utils";

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
    <div className="relative">
      <div className="flex flex-wrap gap-2 mb-6 p-4 bg-white border-2 border-foreground rounded-wobbly shadow-hard rotate-[-0.5deg]">
        <button
          onClick={handleRefresh}
          disabled={loading}
          className={cn(
            "px-4 py-2 font-heading font-bold bg-foreground text-white border-2 border-transparent rounded-wobbly",
            "hover:bg-blue disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          )}
        >
          {loading ? "Refreshing..." : "Refresh RSS"}
        </button>
        <div className="w-px h-8 bg-foreground/20 mx-2 self-center"></div>
        <button
          onClick={() => setFilter("All")}
          className={cn(
            "px-3 py-1 font-sans font-bold border-2 border-transparent hover:border-foreground rounded-full transition-all",
            filter === "All" ? "bg-post-it border-foreground shadow-sm rotate-[-1deg]" : "text-foreground/60"
          )}
        >
          All
        </button>
        {CATEGORIES.map((cat) => (
          <button
            key={cat}
            onClick={() => setFilter(cat)}
            className={cn(
                "px-3 py-1 font-sans font-bold border-2 border-transparent hover:border-foreground rounded-full transition-all",
                filter === cat ? "bg-post-it border-foreground shadow-sm rotate-1" : "text-foreground/60"
            )}
          >
            {cat}
          </button>
        ))}
      </div>

      <div className="space-y-8">
        {filtered.map((article) => (
          <ArticleCard
            key={article.id}
            article={article}
            isFeedbacking={feedbackingId === article.id}
            onSetFeedbackingId={setFeedbackingId}
            onSubmitFeedback={handleSubmitFeedback}
          />
        ))}
        {filtered.length === 0 && (
          <div className="text-center py-20 opacity-60">
            <p className="font-heading text-3xl mb-2">No articles found.</p>
            <p className="font-sans">Try refreshing or changing filters.</p>
          </div>
        )}
      </div>
    </div>
  );
}
