import { FaissStore } from "@langchain/community/vectorstores/faiss";
import { OpenAIEmbeddings } from "@langchain/openai";
import { Document } from "langchain/document";
import fs from "fs";
import logger from "./logger";

const DB_PATH = "./db/faiss_index";

class VectorStoreService {
  private static instance: VectorStoreService;
  private store: FaissStore | null = null;
  private embeddings: OpenAIEmbeddings;

  private constructor() {
    if (!process.env.OPENAI_API_KEY) {
      throw new Error("OPENAI_API_KEY is not set in environment variables.");
    }
    this.embeddings = new OpenAIEmbeddings({
      openAIApiKey: process.env.OPENAI_API_KEY,
    });
    this.load();
  }

  public static getInstance(): VectorStoreService {
    if (!VectorStoreService.instance) {
      VectorStoreService.instance = new VectorStoreService();
    }
    return VectorStoreService.instance;
  }

  private async load() {
    if (fs.existsSync(DB_PATH)) {
      logger.info("Loading existing FAISS index...");
      this.store = await FaissStore.load(DB_PATH, this.embeddings);
      logger.info("FAISS index loaded.");
    } else {
      logger.info("No existing FAISS index found. A new one will be created upon adding documents.");
    }
  }

  public async addDocuments(docs: Document[]) {
    if (!this.store) {
      logger.info("Creating new FAISS index...");
      this.store = await FaissStore.fromDocuments(docs, this.embeddings);
      logger.info("New FAISS index created.");
    } else {
      await this.store.addDocuments(docs);
    }
    await this.save();
  }

  public async save() {
    if (this.store) {
      if (!fs.existsSync("./db")) {
        fs.mkdirSync("./db");
      }
      await this.store.save(DB_PATH);
      logger.info("FAISS index saved to disk.");
    }
  }

  public asRetriever() {
    if (!this.store) {
      // Return a dummy retriever if the store is not initialized
      return {
        getRelevantDocuments: async (query: string) => {
          logger.warn("Vector store not initialized, returning no documents.");
          return [];
        }
      };
    }
    return this.store.asRetriever();
  }
}

export const vectorStoreService = VectorStoreService.getInstance();
