import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { cn } from "../lib/utils";

import { ArticleCategory } from "../types";

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

  if (loading) return <div className="font-sans text-muted-foreground animate-pulse">Loading interests...</div>;

  return (
    <div className={cn("flex flex-wrap gap-3", className)}>
      {categories.map((category, index) => {
        const isSelected = selected.includes(category);
        // Vary rotation slightly based on index
        const rotation = index % 2 === 0 ? "hover:-rotate-1" : "hover:rotate-1";
        const baseRotation = index % 3 === 0 ? "rotate-1" : index % 3 === 1 ? "-rotate-1" : "rotate-0";

        return (
          <button
            key={category}
            onClick={() => toggleCategory(category)}
            className={cn(
              "px-4 py-2 font-sans font-bold text-sm transition-all duration-200 border-2",
              "shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)] active:shadow-none active:translate-x-[2px] active:translate-y-[2px]",
              "rounded-wobbly",
              rotation,
              baseRotation,
              isSelected
                ? "bg-post-it text-foreground border-foreground scale-105"
                : "bg-white text-foreground/60 border-foreground/30 hover:border-foreground hover:bg-white"
            )}
          >
            {isSelected && <span className="mr-1">✓</span>}
            {category}
          </button>
        );
      })}
    </div>
  );
}
