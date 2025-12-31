export type ArticleCategory = "React" | "Rust" | "Android" | "Tauri" | "TypeScript" | "General" | "AI" | "Web" | "Go" | "Python" | "Kotlin";

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
  category: ArticleCategory;
  published_at: string;
  feedback?: Feedback | null;
  image_url?: string;
  author?: string;
}
