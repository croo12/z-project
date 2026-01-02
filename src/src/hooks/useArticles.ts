import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Article } from "../types";

export function useArticles() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);

  const refreshArticles = useCallback(async () => {
    try {
      // Just fetch from DB/cache first
      setArticles(await invoke("get_recommended_articles"));
    } catch (e) {
      console.error("Failed to get recommended articles", e);
    }
  }, []);

  const fetchRss = useCallback(async () => {
    setLoading(true);
    try {
      await invoke("fetch_articles");
      await refreshArticles();
    } catch (e) {
      console.error("Failed to fetch RSS", e);
    } finally {
      setLoading(false);
    }
  }, [refreshArticles]);

  const submitFeedback = useCallback(
    async (id: string, helpful: boolean, reason: string) => {
      try {
        await invoke("submit_feedback", { id, helpful, reason });
        await refreshArticles(); // Refresh to update UI feedback state
      } catch (e) {
        console.error("Failed to submit feedback", e);
      }
    },
    [refreshArticles]
  );

  return { articles, loading, refreshArticles, fetchRss, submitFeedback };
}
