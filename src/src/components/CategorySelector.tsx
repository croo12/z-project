import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { cn } from "../lib/utils";

// Using const object instead of enum to avoid 'erasableSyntaxOnly' issues
export const ArticleCategory = {
  Rust: "Rust",
  Tauri: "Tauri",
  React: "React",
  TypeScript: "TypeScript",
  Android: "Android",
  Kotlin: "Kotlin",
  Web: "Web",
  AI: "AI",
  General: "General",
} as const;

export type ArticleCategory = typeof ArticleCategory[keyof typeof ArticleCategory];

interface CategorySelectorProps {
  className?: string;
}

export function CategorySelector({ className }: CategorySelectorProps) {
  const [selected, setSelected] = useState<ArticleCategory[]>([]);
  const [loading, setLoading] = useState(true);

  const categories = Object.values(ArticleCategory);

  useEffect(() => {
    loadInterests();
  }, []);

  async function loadInterests() {
    try {
      const interests = await invoke<ArticleCategory[]>("get_user_interests");
      setSelected(interests);
    } catch (error) {
      console.error("Failed to load interests:", error);
    } finally {
      setLoading(false);
    }
  }

  async function toggleCategory(category: ArticleCategory) {
    let newSelected;
    if (selected.includes(category)) {
      newSelected = selected.filter((c) => c !== category);
    } else {
      newSelected = [...selected, category];
    }
    setSelected(newSelected);
    
    // Auto-save on toggle
    try {
      await invoke("save_user_interests", { categories: newSelected });
    } catch (error) {
      console.error("Failed to save interests:", error);
    }
  }

  if (loading) return <div>Loading interests...</div>;

  return (
    <div className={cn("flex flex-wrap gap-2", className)}>
      {categories.map((category) => (
        <button
          key={category}
          onClick={() => toggleCategory(category)}
          className={cn(
            "px-3 py-1.5 rounded-full text-sm font-medium transition-colors border",
            selected.includes(category)
              ? "bg-primary text-primary-foreground border-primary"
              : "bg-background text-muted-foreground border-input hover:bg-accent hover:text-accent-foreground"
          )}
        >
          {category}
        </button>
      ))}
    </div>
  );
}
