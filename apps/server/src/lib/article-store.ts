import fs from "fs";
import path from "path";
import { ArticleMetadata, ArticleStore } from "../types/article.js";
import logger from "./logger.js";

const DATA_DIR = path.resolve(process.cwd(), "data");
const ARTICLES_FILE = path.join(DATA_DIR, "articles.json");

class ArticleStoreService {
  private static instance: ArticleStoreService;
  private store: ArticleStore;

  private constructor() {
    this.store = this.loadStore();
  }

  public static getInstance(): ArticleStoreService {
    if (!ArticleStoreService.instance) {
      ArticleStoreService.instance = new ArticleStoreService();
    }
    return ArticleStoreService.instance;
  }

  private loadStore(): ArticleStore {
    try {
      if (!fs.existsSync(DATA_DIR)) {
        fs.mkdirSync(DATA_DIR, { recursive: true });
      }

      if (!fs.existsSync(ARTICLES_FILE)) {
        const emptyStore: ArticleStore = { articles: [] };
        fs.writeFileSync(ARTICLES_FILE, JSON.stringify(emptyStore, null, 2));
        return emptyStore;
      }

      const data = fs.readFileSync(ARTICLES_FILE, "utf-8");
      return JSON.parse(data) as ArticleStore;
    } catch (error) {
      logger.error(error, "Failed to load article store");
      return { articles: [] };
    }
  }

  private saveStore(): void {
    try {
      fs.writeFileSync(ARTICLES_FILE, JSON.stringify(this.store, null, 2));
    } catch (error) {
      logger.error(error, "Failed to save article store");
    }
  }

  public create(article: ArticleMetadata): ArticleMetadata {
    this.store.articles.push(article);
    this.saveStore();
    logger.info(`Article created: ${article.id} - ${article.title}`);
    return article;
  }

  public findById(id: string): ArticleMetadata | undefined {
    return this.store.articles.find((a) => a.id === id);
  }

  public findAll(
    page: number = 1,
    limit: number = 20
  ): { articles: ArticleMetadata[]; total: number } {
    const start = (page - 1) * limit;
    const end = start + limit;
    return {
      articles: this.store.articles.slice(start, end),
      total: this.store.articles.length,
    };
  }

  public update(
    id: string,
    updates: Partial<ArticleMetadata>
  ): ArticleMetadata | undefined {
    const index = this.store.articles.findIndex((a) => a.id === id);
    if (index === -1) {
      return undefined;
    }

    this.store.articles[index] = {
      ...this.store.articles[index],
      ...updates,
      updatedAt: new Date().toISOString(),
    };
    this.saveStore();
    logger.info(`Article updated: ${id}`);
    return this.store.articles[index];
  }

  public delete(id: string): boolean {
    const index = this.store.articles.findIndex((a) => a.id === id);
    if (index === -1) {
      return false;
    }

    this.store.articles.splice(index, 1);
    this.saveStore();
    logger.info(`Article deleted: ${id}`);
    return true;
  }

  public updateFeedback(
    id: string,
    type: "positive" | "negative"
  ): ArticleMetadata | undefined {
    const article = this.findById(id);
    if (!article) {
      return undefined;
    }

    if (type === "positive") {
      article.positiveCount += 1;
    } else {
      article.negativeCount += 1;
    }

    // Calculate new rating
    const totalFeedback = article.positiveCount + article.negativeCount;
    if (totalFeedback > 0) {
      // Formula: (positiveCount - negativeCount * 0.5) / totalFeedbackCount + 1.0
      const rawRating =
        (article.positiveCount - article.negativeCount * 0.5) / totalFeedback +
        1.0;
      // Clamp to 0.0 ~ 2.0
      article.rating = Math.max(0.0, Math.min(2.0, rawRating));
    }

    return this.update(id, {
      rating: article.rating,
      positiveCount: article.positiveCount,
      negativeCount: article.negativeCount,
    });
  }
}

export const articleStoreService = ArticleStoreService.getInstance();
