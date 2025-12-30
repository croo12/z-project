import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import ArticleList from "../components/ArticleList";
import type { Article } from "../types";
import "../App.css";

export default function ArticleView() {
  const [articles, setArticles] = useState<Article[]>([]);

  useEffect(() => {
    refreshArticles();
  }, []);

  async function refreshArticles() {
    console.log("Refreshing articles...");
    try {
        const data = await invoke<Article[]>("get_recommended_articles");
        setArticles(data);
    } catch (e) {
        console.error("Failed to fetch articles", e);
    }
  }

  return (
    <div className="view-container">
      <h2>Dev Recommendations</h2>
      <ArticleList 
        articles={articles} 
        onRefresh={refreshArticles} 
        onFeedbackUpdate={refreshArticles} 
      />
    </div>
  );
}
