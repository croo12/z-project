import "dotenv/config"; // Load environment variables first
import express from "express";
import knowledgeRouter from "./api/knowledge.js";
import interactionsRouter from "./api/interactions.js";
import feedbackRouter from "./api/feedback.js";
import queryRouter from "./api/query.js";
import articlesRouter from "./api/articles.js";
import { vectorStoreService } from "./lib/vector-store.js";
import { articleStoreService } from "./lib/article-store.js";
import logger from "./lib/logger.js";

// Environment variable validation
const requiredEnvVars = ["GEMINI_API_KEY"];
for (const envVar of requiredEnvVars) {
  if (!process.env[envVar]) {
    logger.error(`Missing required environment variable: ${envVar}`);
    process.exit(1);
  }
}
logger.info("Environment variables validated successfully.");

// Initialize services
logger.info("Initializing services...");
await Promise.all([
  vectorStoreService.initialize(),
  articleStoreService.initialize(),
]).catch((e) => {
  logger.error(e, "Failed to initialize services.");
  process.exit(1);
});
logger.info("Services initialized successfully.");

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

app.get("/", (req, res) => {
  res.send("AI Brain Server is running!");
});

app.use("/knowledge", knowledgeRouter);
app.use("/interactions", interactionsRouter);
app.use("/feedback", feedbackRouter);
app.use("/query", queryRouter);
app.use("/articles", articlesRouter);

const host = "0.0.0.0";

app.listen(Number(port), host, () => {
  logger.info(`Server is listening on ${host}:${port}`);
});
