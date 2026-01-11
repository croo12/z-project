import { Router, Request, Response, NextFunction } from "express";
import { articleService } from "../services/article.service.js";
import { CreateArticleInput } from "../types/article.js";
import logger from "../lib/logger.js";

const router = Router();

// POST /articles - Create a new article
router.post("/", async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { title, url, content, tags } = req.body as CreateArticleInput;

    // Validation
    if (
      !title ||
      typeof title !== "string" ||
      title.length < 1 ||
      title.length > 200
    ) {
      return res
        .status(400)
        .json({
          error: "Invalid title",
          message: "title은 1~200자 문자열이어야 합니다.",
        });
    }

    if (!url || typeof url !== "string" || !url.match(/^https?:\/\/.+/)) {
      return res
        .status(400)
        .json({
          error: "Invalid url",
          message:
            "url은 http:// 또는 https://로 시작하는 유효한 URL이어야 합니다.",
        });
    }

    if (!content || typeof content !== "string" || content.length < 10) {
      return res
        .status(400)
        .json({
          error: "Invalid content",
          message: "content는 10자 이상의 문자열이어야 합니다.",
        });
    }

    if (
      tags &&
      (!Array.isArray(tags) ||
        tags.length > 10 ||
        tags.some((t: string) => typeof t !== "string" || t.length > 50))
    ) {
      return res
        .status(400)
        .json({
          error: "Invalid tags",
          message: "tags는 최대 10개, 각 태그는 50자 이하여야 합니다.",
        });
    }

    const article = await articleService.createArticle({
      title,
      url,
      content,
      tags,
    });
    res.status(201).json(article);
  } catch (error) {
    logger.error(error, "Error creating article");
    next(error);
  }
});

// GET /articles - List articles with pagination
router.get("/", (req: Request, res: Response, next: NextFunction) => {
  try {
    const page = Math.max(1, parseInt(req.query.page as string) || 1);
    const limit = Math.min(
      100,
      Math.max(1, parseInt(req.query.limit as string) || 20)
    );

    const result = articleService.listArticles(page, limit);
    res.json(result);
  } catch (error) {
    logger.error(error, "Error listing articles");
    next(error);
  }
});

// GET /articles/:id - Get article by ID
router.get("/:id", (req: Request, res: Response, next: NextFunction) => {
  try {
    const article = articleService.getArticleById(req.params.id);
    if (!article) {
      return res
        .status(404)
        .json({ error: "Not found", message: "아티클을 찾을 수 없습니다." });
    }
    res.json(article);
  } catch (error) {
    logger.error(error, "Error getting article");
    next(error);
  }
});

// DELETE /articles/:id - Delete article
router.delete("/:id", (req: Request, res: Response, next: NextFunction) => {
  try {
    const deleted = articleService.deleteArticle(req.params.id);
    if (!deleted) {
      return res
        .status(404)
        .json({ error: "Not found", message: "아티클을 찾을 수 없습니다." });
    }
    res.status(204).send();
  } catch (error) {
    logger.error(error, "Error deleting article");
    next(error);
  }
});

// POST /articles/:id/feedback - Submit feedback for an article
router.post(
  "/:id/feedback",
  async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { type, comment } = req.body;

      if (!type || !["positive", "negative"].includes(type)) {
        return res
          .status(400)
          .json({
            error: "Invalid type",
            message: "type은 positive 또는 negative여야 합니다.",
          });
      }

      const article = await articleService.submitFeedback(req.params.id, type);
      if (!article) {
        return res
          .status(404)
          .json({ error: "Not found", message: "아티클을 찾을 수 없습니다." });
      }

      res.json({
        message: "피드백이 제출되었습니다.",
        article: {
          id: article.id,
          rating: article.rating,
          positiveCount: article.positiveCount,
          negativeCount: article.negativeCount,
        },
      });
    } catch (error) {
      logger.error(error, "Error submitting feedback");
      next(error);
    }
  }
);

// GET /articles/:id/feedback - Get feedback stats for an article
router.get(
  "/:id/feedback",
  (req: Request, res: Response, next: NextFunction) => {
    try {
      const stats = articleService.getFeedbackStats(req.params.id);
      if (!stats) {
        return res
          .status(404)
          .json({ error: "Not found", message: "아티클을 찾을 수 없습니다." });
      }
      res.json(stats);
    } catch (error) {
      logger.error(error, "Error getting feedback stats");
      next(error);
    }
  }
);

export default router;
