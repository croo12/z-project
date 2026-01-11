import { RecursiveCharacterTextSplitter } from "@langchain/textsplitters";
import { Document } from "@langchain/core/documents";
import { vectorStoreService } from "../lib/vector-store.js";
import { articleStoreService } from "../lib/article-store.js";
import {
  ArticleMetadata,
  CreateArticleInput,
  ArticleFeedbackStats,
} from "../types/article.js";
import logger from "../lib/logger.js";
import crypto from "crypto";

export class ArticleService {
  private textSplitter: RecursiveCharacterTextSplitter;

  constructor() {
    this.textSplitter = new RecursiveCharacterTextSplitter({
      chunkSize: 1000,
      chunkOverlap: 200,
    });
  }

  async createArticle(input: CreateArticleInput): Promise<ArticleMetadata> {
    const articleId = crypto.randomUUID();
    const now = new Date().toISOString();

    logger.info(`Creating article: ${input.title}`);

    // Split content into chunks
    const chunks = await this.textSplitter.splitText(input.content);

    // Create documents with article metadata
    const documents = chunks.map(
      (chunk: string) =>
        new Document({
          id: crypto.randomUUID(),
          pageContent: chunk,
          metadata: {
            source: input.url,
            articleId,
            articleTitle: input.title,
            ingested_at: now, // Use ISO string instead of Date object
            retrieval_score_modifier: 1.0,
          },
        })
    );

    // Store in vector DB
    await vectorStoreService.addDocuments(documents);

    // Create article metadata
    const article: ArticleMetadata = {
      id: articleId,
      title: input.title,
      url: input.url,
      tags: input.tags || [],
      createdAt: now,
      updatedAt: now,
      chunkCount: chunks.length,
      rating: 1.0,
      positiveCount: 0,
      negativeCount: 0,
    };

    // Save to article store
    return articleStoreService.create(article);
  }

  getArticleById(id: string): ArticleMetadata | undefined {
    return articleStoreService.findById(id);
  }

  listArticles(page: number = 1, limit: number = 20) {
    const { articles, total } = articleStoreService.findAll(page, limit);
    return {
      articles,
      pagination: {
        page,
        limit,
        total,
        totalPages: Math.ceil(total / limit),
      },
    };
  }

  deleteArticle(id: string): boolean {
    return articleStoreService.delete(id);
  }

  async submitFeedback(
    articleId: string,
    type: "positive" | "negative"
  ): Promise<ArticleMetadata | undefined> {
    const article = articleStoreService.updateFeedback(articleId, type);

    if (article) {
      // Update vector scores for all chunks of this article
      await this.updateVectorScores(articleId, article.rating);
      logger.info(
        `Feedback submitted for article ${articleId}: ${type}, new rating: ${article.rating}`
      );
    }

    return article;
  }

  private async updateVectorScores(
    articleId: string,
    newScore: number
  ): Promise<void> {
    try {
      await vectorStoreService.updateScoresByArticleId(articleId, newScore);
    } catch (error) {
      logger.error(
        error,
        `Failed to update vector scores for article ${articleId}`
      );
    }
  }

  getFeedbackStats(articleId: string): ArticleFeedbackStats | undefined {
    const article = articleStoreService.findById(articleId);
    if (!article) {
      return undefined;
    }

    return {
      articleId: article.id,
      rating: article.rating,
      positiveCount: article.positiveCount,
      negativeCount: article.negativeCount,
      totalFeedbackCount: article.positiveCount + article.negativeCount,
      retrievalScoreModifier: article.rating,
    };
  }
}

export const articleService = new ArticleService();
