import { memo } from "react";
import type { Article, ArticleCategory } from "../types";
import FeedbackForm from "./FeedbackForm";
import { cn } from "../lib/utils";

interface ArticleCardProps {
  article: Article;
  isFeedbacking: boolean;
  onSetFeedbackingId: (id: string | null) => void;
  onSubmitFeedback: (id: string, helpful: boolean, reason: string) => void;
}

function getCategoryColorClass(cat: ArticleCategory): string {
  switch (cat) {
    case "Rust":
      return "text-[#dea584] border-[#dea584]";
    case "React":
      return "text-[#61dafb] border-[#61dafb]";
    case "Android":
      return "text-[#3ddc84] border-[#3ddc84]";
    case "Tauri":
      return "text-[#ffc131] border-[#ffc131]";
    case "TypeScript":
      return "text-[#3178c6] border-[#3178c6]";
    default:
      return "text-foreground border-foreground";
  }
}

const ArticleCard = memo(function ArticleCard({
  article,
  isFeedbacking,
  onSetFeedbackingId,
  onSubmitFeedback,
}: ArticleCardProps) {

  return (
    <div className={cn(
      "relative bg-white border-[3px] border-foreground p-6 mb-8 transition-all duration-300",
      "shadow-hard hover:shadow-hard-hover hover:-translate-y-1 hover:rotate-1",
      "rounded-wobbly"
    )}>
      {/* Tape Decoration */}
      <div className="absolute -top-3 left-1/2 -translate-x-1/2 w-32 h-8 bg-gray-200/50 rotate-[-2deg] pointer-events-none z-10"></div>

      <div className="flex flex-wrap gap-2 mb-4">
        {article.tags.map((tag) => (
          <span
            key={tag}
            className={cn(
              "px-2 py-0.5 text-xs font-bold font-sans border border-current rounded-full bg-white",
              getCategoryColorClass(tag)
            )}
          >
            #{tag}
          </span>
        ))}
        <span className="ml-auto text-xs text-foreground/60 font-sans">
          {new Date(article.published_at).toLocaleDateString()}
        </span>
      </div>

      <h3 className="text-2xl font-heading font-bold mb-3 leading-tight">{article.title}</h3>

      {article.image_url && (
        <div className="mb-4 relative border-2 border-foreground p-1 bg-white rounded-wobbly rotate-[-1deg]">
            <img
            src={article.image_url}
            alt={article.title}
            loading="lazy"
            className="w-full h-40 object-cover rounded-wobbly"
            />
        </div>
      )}

      <p className="font-sans text-lg leading-relaxed mb-4 text-foreground/90">{article.summary}</p>

      {article.author && (
        <small className="block text-foreground/60 font-sans mb-4 italic">
          — {article.author}
        </small>
      )}

      <a
        href={article.url}
        target="_blank"
        className="inline-block font-heading font-bold text-blue hover:text-accent decoration-wavy underline text-lg mb-6 transition-colors"
        rel="noreferrer"
      >
        Read Article &rarr;
      </a>

      {/* Feedback UI */}
      <div className="pt-4 border-t-2 border-dashed border-foreground/20">
        {article.feedback ? (
          <div className="font-sans text-sm">
            <span className="font-bold">Feedback:</span>{" "}
            {article.feedback.is_helpful ? "👍 Helpful" : "👎 Not Helpful"}
            {article.feedback.reason && <span className="text-foreground/60"> ({article.feedback.reason})</span>}
          </div>
        ) : isFeedbacking ? (
          <FeedbackForm
            onSubmit={(helpful, reason) =>
              onSubmitFeedback(article.id, helpful, reason)
            }
            onCancel={() => onSetFeedbackingId(null)}
          />
        ) : (
          <button
            onClick={() => onSetFeedbackingId(article.id)}
            className="text-sm font-sans font-bold text-foreground/60 hover:text-blue hover:underline decoration-dashed"
          >
            Rate this article
          </button>
        )}
      </div>
    </div>
  );
});

export default ArticleCard;
