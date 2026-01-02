import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { type Article, ArticleCategory } from "../types";

export function useArticles() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      // First fetch/update from RSS
      await invoke("fetch_articles");
      // Then get recommended
      const data = await invoke<Article[]>("get_recommended_articles");
      setArticles(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const submitFeedback = useCallback(
    async (id: string, helpful: boolean, reason: string) => {
      try {
        await invoke("submit_feedback", { id, helpful, reason });
        // Refresh to update UI (remove feedbacked item)
        await refresh();
      } catch (err) {
        console.error("Failed to submit feedback", err);
      }
    },
    [refresh]
  );

  const saveInterests = useCallback(async (categories: ArticleCategory[]) => {
    await invoke("save_user_interests", { categories });
  }, []);

  const getInterests = useCallback(async () => {
    return await invoke<ArticleCategory[]>("get_user_interests");
  }, []);

  // Initial fetch
  useEffect(() => {
    refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    articles,
    loading,
    error,
    refresh,
    submitFeedback,
    saveInterests,
    getInterests,
  };
}
