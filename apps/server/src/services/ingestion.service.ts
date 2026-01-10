import { RecursiveCharacterTextSplitter } from "langchain/text_splitter";
import { Document } from "langchain/document";
import { vectorStoreService } from "../lib/vector-store";
import logger from "../lib/logger";

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

    const documents = chunks.map((chunk) => new Document({
      pageContent: chunk,
      metadata: {
        source,
        ingested_at: new Date(),
      },
    }));

    await vectorStoreService.addDocuments(documents);
    logger.info(`Successfully ingested ${documents.length} chunks from source: ${source}`);
  }
}

export const ingestionService = new IngestionService();
