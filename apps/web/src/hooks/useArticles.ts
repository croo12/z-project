import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Article } from "../types";

export function useArticles() {
  const [articles, setArticles] = useState<Article[]>([]);

  const refreshArticles = useCallback(async () => {
    console.log("Refreshing articles...");
    try {
      const data = await invoke<Article[]>("get_recommended_articles");
      setArticles(data);
    } catch (e) {
      console.error("Failed to fetch articles", e);
    }
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refreshArticles();
  }, [refreshArticles]);

  return { articles, refreshArticles };
}
