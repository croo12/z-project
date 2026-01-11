/**
 * Article 관련 타입 정의
 */

export interface ArticleMetadata {
  id: string;
  title: string;
  url: string;
  tags: string[];
  createdAt: string;
  updatedAt: string;
  chunkCount: number;
  rating: number;
  positiveCount: number;
  negativeCount: number;
}

export interface ArticleStore {
  articles: ArticleMetadata[];
}

export interface CreateArticleInput {
  title: string;
  url: string;
  content: string;
  tags?: string[];
}

export interface ArticleFeedback {
  id: string;
  articleId: string;
  type: "positive" | "negative";
  comment?: string;
  createdAt: string;
}

export interface ArticleListResponse {
  articles: ArticleMetadata[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

export interface ArticleFeedbackStats {
  articleId: string;
  rating: number;
  positiveCount: number;
  negativeCount: number;
  totalFeedbackCount: number;
  retrievalScoreModifier: number;
}
