import { RecursiveCharacterTextSplitter } from "@langchain/textsplitters";
import { Document } from "@langchain/core/documents";
import { vectorStoreService } from "../lib/vector-store.js";
import logger from "../lib/logger.js";
import crypto from "crypto";

export class IngestionService {
  private textSplitter: RecursiveCharacterTextSplitter;

  constructor() {
    this.textSplitter = new RecursiveCharacterTextSplitter({
      chunkSize: 1000,
      chunkOverlap: 200,
    });
  }

  async ingest(content: string, source: string): Promise<void> {
    logger.info(`Ingesting content from source: ${source}`);

    const chunks = await this.textSplitter.splitText(content);

    const documents = chunks.map(
      (chunk: string) =>
        new Document({
          id: crypto.randomUUID(),
          pageContent: chunk,
          metadata: {
            source,
            ingested_at: new Date().toISOString(),
            retrieval_score_modifier: 1.0,
          },
        })
    );

    await vectorStoreService.addDocuments(documents);
    logger.info(
      `Successfully ingested ${documents.length} chunks from source: ${source}`
    );
  }

  async ingestWithArticleMetadata(
    content: string,
    source: string,
    articleId: string,
    articleTitle: string,
    ingestedAt: string
  ): Promise<Document[]> {
    const chunks = await this.textSplitter.splitText(content);

    return chunks.map(
      (chunk: string) =>
        new Document({
          id: crypto.randomUUID(),
          pageContent: chunk,
          metadata: {
            source,
            articleId,
            articleTitle,
            ingested_at: ingestedAt,
            retrieval_score_modifier: 1.0,
          },
        })
    );
  }
}

export const ingestionService = new IngestionService();
