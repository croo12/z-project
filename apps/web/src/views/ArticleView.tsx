import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import ArticleList from "../components/ArticleList";
import { CategorySelector } from "../components/CategorySelector";
import type { Article } from "../types";
import { cn } from "../lib/utils";

export default function ArticleView() {
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
    refreshArticles();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="flex flex-col gap-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-4xl font-heading font-bold text-foreground rotate-[-1deg]">
          Dev Recommendations
        </h2>
        <button 
          onClick={() => setShowSettings(!showSettings)}
          className={cn(
            "p-3 rounded-full border-2 border-foreground bg-white transition-all duration-200",
            "shadow-hard hover:shadow-hard-hover hover:translate-x-[2px] hover:translate-y-[2px]",
            "active:shadow-none active:translate-x-[4px] active:translate-y-[4px]",
            showSettings ? "bg-accent text-white border-accent" : "text-foreground"
          )}
          title="Manage Interests"
          aria-label="Toggle interests settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
        </button>
      </div>
      
      {showSettings && (
        <div className="mb-6 p-6 bg-white border-2 border-dashed border-foreground/30 rounded-wobbly rotate-1">
          <h3 className="text-xl font-heading font-bold text-foreground mb-4">Filter by Interests</h3>
          <CategorySelector />
        </div>
      )}

      <ArticleList
        articles={articles}
        onRefresh={refreshArticles}
        onFeedbackUpdate={refreshArticles}
      />
    </div>
  );
}
