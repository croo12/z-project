import { StateGraph, END } from "@langchain/langgraph";
import { Document } from "@langchain/core/documents";
import { vectorStoreService } from "../lib/vector-store.js";
import { createChatModel } from "@z-project/ai";
import {
  BaseChatModel,
  BaseChatModelCallOptions,
} from "@langchain/core/language_models/chat_models";
import {
  RunnablePassthrough,
  RunnableSequence,
} from "@langchain/core/runnables";
import { StringOutputParser } from "@langchain/core/output_parsers";
import { PromptTemplate } from "@langchain/core/prompts";
import logger from "../lib/logger.js";
import { BaseMessageChunk } from "@langchain/core/messages";

// The state of our graph
export interface AgentState {
  query: string;
  context?: any;
  documents: Document[];
  response: string;
}

const formatDocuments = (docs: Document[]) => {
  return docs.map((doc) => doc.pageContent).join("\n\n");
};

const RAG_PROMPT_TEMPLATE = `You are a helpful assistant. Use the following context to answer the question. If you don't know the answer, just say that you don't know.

Context:
{context}

Question:
{question}`;

class RAGGraph {
  private workflow: StateGraph<AgentState>;
  private llm: BaseChatModel;

  constructor() {
    this.llm = createChatModel({
      provider: "gemini",
      modelName: "gemini-2.5-flash",
      temperature: 0,
      apiKey: process.env.GEMINI_API_KEY,
    });

    this.workflow = new StateGraph<AgentState>({
      channels: {
        query: {
          value: (x, y) => y,
          default: () => "",
        },
        context: {
          value: (x, y) => y,
          default: () => undefined,
        },
        documents: {
          value: (x, y) => x.concat(y),
          default: () => [],
        },
        response: {
          value: (x, y) => y,
          default: () => "",
        },
      },
    });

    this.buildGraph();
  }

  private buildGraph() {
    this.workflow
      .addNode("retrieve", this.retrieve.bind(this))
      .addNode("generate", this.generate.bind(this))
      .setEntryPoint("retrieve")
      .addEdge("retrieve", "generate")
      .addEdge("generate", END);
  }

  private async retrieve(state: AgentState): Promise<Partial<AgentState>> {
    logger.info({ query: state.query }, "---RETRIEVING DOCUMENTS---");
    const retriever = vectorStoreService.asRetriever();
    const documents = await retriever.invoke(state.query);
    logger.info(`Retrieved ${documents.length} documents.`);
    return { documents };
  }

  private async generate(state: AgentState): Promise<Partial<AgentState>> {
    logger.info("---GENERATING RESPONSE---");
    const prompt = PromptTemplate.fromTemplate(RAG_PROMPT_TEMPLATE);

    const ragChain = RunnableSequence.from([
      {
        context: () => formatDocuments(state.documents),
        question: new RunnablePassthrough(),
      },
      prompt,
      this.llm,
      new StringOutputParser(),
    ]);

    const response = await ragChain.invoke(state.query);

    return { response };
  }

  public compile() {
    return this.workflow.compile();
  }
}

export const ragGraph = new RAGGraph().compile();
