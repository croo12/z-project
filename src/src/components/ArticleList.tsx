import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Article, ArticleCategory } from "../types";
import "../App.css";

const CATEGORIES: ArticleCategory[] = ["React", "Rust", "Android", "Tauri", "TypeScript", "General"];

interface Props {
  articles: Article[];
  onRefresh: () => void;
  onFeedbackUpdate: () => void;
}

export default function ArticleList({ articles, onRefresh, onFeedbackUpdate }: Props) {
  const [filter, setFilter] = useState<ArticleCategory | "All">("All");
  const [loading, setLoading] = useState(false);
  const [feedbackingId, setFeedbackingId] = useState<string | null>(null);
  const [reason, setReason] = useState("");

  const filtered = filter === "All" ? articles : articles.filter((a) => a.category === filter);

  async function handleRefresh() {
    setLoading(true);
    try {
      await invoke("fetch_articles");
      onRefresh();
    } finally {
      setLoading(false);
    }
  }

  async function handleSubmitFeedback(id: string, helpful: boolean) {
    if (!reason.trim()) {
      alert("Please provide a reason.");
      return;
    }
    await invoke("submit_feedback", { id, helpful, reason });
    setFeedbackingId(null);
    setReason("");
    onFeedbackUpdate();
  }

  return (
    <div className="article-list-container">
        <div className="controls" style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', marginBottom: '1rem' }}>
            <button onClick={handleRefresh} disabled={loading}>{loading ? "Refreshing..." : "Refresh RSS"}</button>
            <button onClick={() => setFilter("All")} className={filter === "All" ? "active" : ""}>All</button>
            {CATEGORIES.map(cat => (
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

        <div className="articles-grid" style={{ display: 'grid', gap: '1rem' }}>
            {filtered.map(article => (
                <div key={article.id} className="card article-item" style={{ borderLeft: `4px solid ${getCategoryColor(article.category)}` }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                        <span style={{ fontSize: '0.8rem', fontWeight: 'bold', color: getCategoryColor(article.category) }}>
                            {article.category}
                        </span>
                        <span style={{ fontSize: '0.8rem', color: '#888' }}>
                            {new Date(article.published_at).toLocaleDateString()}
                        </span>
                    </div>
                    <h3>{article.title}</h3>
                    <p>{article.summary}</p>
                    <a href={article.url} target="_blank" className="news-link">Read Article &rarr;</a>
                    
                    {/* Feedback UI */}
                    <div className="feedback-section" style={{ marginTop: '1rem', borderTop: '1px solid #eee', paddingTop: '0.5rem' }}>
                        {article.feedback ? (
                            <small>
                                Feedback: {article.feedback.is_helpful ? "👍 Helpful" : "👎 Not Helpful"} 
                                ({article.feedback.reason})
                            </small>
                        ) : (
                            feedbackingId === article.id ? (
                                <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.5rem' }}>
                                    <input 
                                        value={reason} 
                                        onChange={e => setReason(e.target.value)} 
                                        placeholder="Reason..." 
                                        autoFocus
                                    />
                                    <button onClick={() => handleSubmitFeedback(article.id, true)}>👍</button>
                                    <button onClick={() => handleSubmitFeedback(article.id, false)}>👎</button>
                                    <button onClick={() => setFeedbackingId(null)} style={{ background: '#888' }}>Cancel</button>
                                </div>
                            ) : (
                                <button 
                                    onClick={() => setFeedbackingId(article.id)} 
                                    style={{ fontSize: '0.8rem', padding: '4px 8px', background: 'transparent', color: '#888', border: '1px solid #eee' }}
                                >
                                    Rate this article
                                </button>
                            )
                        )}
                    </div>
                </div>
            ))}
            {filtered.length === 0 && <p>No articles found.</p>}
        </div>
    </div>
  );
}

function getCategoryColor(cat: ArticleCategory): string {
    switch (cat) {
        case "Rust": return "#dea584";
        case "React": return "#61dafb";
        case "Android": return "#3ddc84";
        case "Tauri": return "#ffc131";
        case "TypeScript": return "#3178c6";
        default: return "#888";
    }
}
