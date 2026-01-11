import fs from "fs";
import path from "path";
import { ArticleMetadata, ArticleStore } from "../types/article.js";
import logger from "./logger.js";

const DATA_DIR = path.resolve(process.cwd(), "data");
const ARTICLES_FILE = path.join(DATA_DIR, "articles.json");

class ArticleStoreService {
  private static instance: ArticleStoreService;
  private store: ArticleStore;
  private isLoaded: boolean = false;

  private constructor() {
    this.store = { articles: [] };
  }

  public static getInstance(): ArticleStoreService {
    if (!ArticleStoreService.instance) {
      ArticleStoreService.instance = new ArticleStoreService();
    }
    return ArticleStoreService.instance;
  }

  public async initialize(): Promise<void> {
    if (this.isLoaded) return;
    this.store = await this.loadStore();
    this.isLoaded = true;
  }

  private async loadStore(): Promise<ArticleStore> {
    try {
      if (!fs.existsSync(DATA_DIR)) {
        await fs.promises.mkdir(DATA_DIR, { recursive: true });
      }

      if (!fs.existsSync(ARTICLES_FILE)) {
        const emptyStore: ArticleStore = { articles: [] };
        await fs.promises.writeFile(
          ARTICLES_FILE,
          JSON.stringify(emptyStore, null, 2)
        );
        return emptyStore;
      }

      const data = await fs.promises.readFile(ARTICLES_FILE, "utf-8");
      return JSON.parse(data) as ArticleStore;
    } catch (error) {
      logger.error(error, "Failed to load article store");
      return { articles: [] };
    }
  }

  private async saveStore(): Promise<void> {
    try {
      await fs.promises.writeFile(
        ARTICLES_FILE,
        JSON.stringify(this.store, null, 2)
      );
    } catch (error) {
      logger.error(error, "Failed to save article store");
    }
  }

  public async create(article: ArticleMetadata): Promise<ArticleMetadata> {
    this.store.articles.push(article);
    await this.saveStore();
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

  public async update(
    id: string,
    updates: Partial<ArticleMetadata>
  ): Promise<ArticleMetadata | undefined> {
    const index = this.store.articles.findIndex((a) => a.id === id);
    if (index === -1) {
      return undefined;
    }

    this.store.articles[index] = {
      ...this.store.articles[index],
      ...updates,
      updatedAt: new Date().toISOString(),
    };
    await this.saveStore();
    logger.info(`Article updated: ${id}`);
    return this.store.articles[index];
  }

  public async delete(id: string): Promise<boolean> {
    const index = this.store.articles.findIndex((a) => a.id === id);
    if (index === -1) {
      return false;
    }

    this.store.articles.splice(index, 1);
    await this.saveStore();
    logger.info(`Article deleted: ${id}`);
    return true;
  }

  public async updateFeedback(
    id: string,
    type: "positive" | "negative"
  ): Promise<ArticleMetadata | undefined> {
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

    return await this.update(id, {
      rating: article.rating,
      positiveCount: article.positiveCount,
      negativeCount: article.negativeCount,
    });
  }
}

export const articleStoreService = ArticleStoreService.getInstance();
