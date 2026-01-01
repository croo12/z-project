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

export interface Feedback {
  is_helpful: boolean;
  reason: string;
  created_at: string;
}

export interface Article {
  id: string;
  title: string;
  summary: string;
  url: string;
  tags: ArticleCategory[];
  published_at: string;
  feedback?: Feedback | null;
  image_url?: string;
  author?: string;
}
