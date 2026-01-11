import { LanceDB } from "@langchain/community/vectorstores/lancedb";
import { VectorStore } from "@langchain/core/vectorstores";
import { GoogleGenerativeAIEmbeddings } from "@langchain/google-genai";
import { Document } from "@langchain/core/documents";
import * as lancedb from "@lancedb/lancedb";
import fs from "fs";
import logger from "./logger.js";

const DB_PATH = "./db";
const TABLE_NAME = "knowledge_store";

class VectorStoreService {
  private static instance: VectorStoreService;
  private store: LanceDB | null = null;
  private embeddings: GoogleGenerativeAIEmbeddings;
  private db: lancedb.Connection | null = null;
  private table: lancedb.Table | null = null;

  private constructor() {
    if (!process.env.GEMINI_API_KEY) {
      throw new Error("GEMINI_API_KEY is not set in environment variables.");
    }
    this.embeddings = new GoogleGenerativeAIEmbeddings({
      apiKey: process.env.GEMINI_API_KEY,
      modelName: "text-embedding-004", // Updated Gemini embedding model
    });
  }

  public static getInstance(): VectorStoreService {
    if (!VectorStoreService.instance) {
      VectorStoreService.instance = new VectorStoreService();
    }
    return VectorStoreService.instance;
  }

  public async initialize(): Promise<void> {
    await this.init();
  }

  private async init() {
    try {
      // Ensure DB directory exists
      if (!fs.existsSync(DB_PATH)) {
        fs.mkdirSync(DB_PATH, { recursive: true });
      }

      this.db = await lancedb.connect(DB_PATH);

      const tableNames = await this.db.tableNames();
      if (tableNames.includes(TABLE_NAME)) {
        this.table = await this.db.openTable(TABLE_NAME);
        this.store = new LanceDB(this.embeddings, { table: this.table });
        logger.info("LanceDB loaded and connected to table.");
      } else {
        logger.info(
          "LanceDB table does not exist yet. It will be created when documents are added."
        );
      }
    } catch (error) {
      logger.error(error, "Failed to initialize LanceDB.");
    }
  }

  public async addDocuments(docs: Document[]) {
    // If store already exists, add to it
    if (this.store) {
      await this.store.addDocuments(docs);
      logger.info(`Added ${docs.length} documents to existing LanceDB table.`);
    } else {
      // Create new table
      // We rely on the static fromDocuments to create the table and initialize everything
      logger.info("Creating new LanceDB table from documents...");
      this.store = await LanceDB.fromDocuments(docs, this.embeddings, {
        uri: DB_PATH,
        tableName: TABLE_NAME,
      });

      // Refresh our references
      this.db = await lancedb.connect(DB_PATH);
      this.table = await this.db.openTable(TABLE_NAME);
      logger.info("New LanceDB index created.");
    }
  }

  /**
   * Updates the retrieval_score_modifier for a specific document.
   * @param documentId The unique ID of the document (metadata.id)
   * @param newScore The new score value
   */
  public async updateScore(documentId: string, newScore: number) {
    if (!this.table) {
      // Try to re-init if table was just created
      if (this.db) {
        const tableNames = await this.db.tableNames();
        if (tableNames.includes(TABLE_NAME)) {
          this.table = await this.db.openTable(TABLE_NAME);
        }
      }
    }

    if (!this.table) {
      logger.warn("LanceDB table not initialized. Cannot update score.");
      return;
    }

    try {
      // LanceDB SQL-like update. we assume 'id' column exists from metadata.
      // We need to verify if 'id' is in metadata or top level.
      // LangChain flattens metadata into columns.
      // So metadata: { id: "123" } -> column "id".
      await this.table.update({
        where: `id = '${documentId}'`,
        values: { retrieval_score_modifier: newScore },
      });
      logger.info(`Updated score for document ${documentId} to ${newScore}`);
    } catch (error) {
      logger.error(error, `Failed to update score for document ${documentId}`);
    }
  }

  /**
   * Updates the retrieval_score_modifier for all documents belonging to an article.
   * @param articleId The article ID
   * @param newScore The new score value
   */
  public async updateScoresByArticleId(articleId: string, newScore: number) {
    if (!this.table) {
      if (this.db) {
        const tableNames = await this.db.tableNames();
        if (tableNames.includes(TABLE_NAME)) {
          this.table = await this.db.openTable(TABLE_NAME);
        }
      }
    }

    if (!this.table) {
      logger.warn("LanceDB table not initialized. Cannot update scores.");
      return;
    }

    try {
      // Update all documents with matching articleId
      await this.table.update({
        where: `articleId = '${articleId}'`,
        values: { retrieval_score_modifier: newScore },
      });
      logger.info(`Updated scores for article ${articleId} to ${newScore}`);
    } catch (error) {
      logger.error(error, `Failed to update scores for article ${articleId}`);
    }
  }

  public asRetriever() {
    if (!this.store) {
      return {
        invoke: async (input: string, options?: any) => {
          logger.warn("Vector store not initialized, returning no documents.");
          return [];
        },
        getRelevantDocuments: async (query: string) => {
          logger.warn("Vector store not initialized, returning no documents.");
          return [];
        },
      };
    }
    return (this.store as VectorStore).asRetriever();
  }
}

export const vectorStoreService = VectorStoreService.getInstance();
