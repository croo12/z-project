import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Article } from "../types";

export function useArticles() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);

  const refreshArticles = useCallback(async () => {
    setLoading(true);
    try {
      const data = await invoke<Article[]>("get_recommended_articles");
      setArticles(data);
    } catch (e) {
      console.error("Failed to fetch articles", e);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refreshArticles();
  }, [refreshArticles]);

  return { articles, loading, refreshArticles };
}
