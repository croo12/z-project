import { ChatOpenAI } from "@langchain/openai";
import { BaseChatModel } from "@langchain/core/language_models/chat_models";
import { AIModelConfig } from "../types/index.js";

export function createOpenAIModel(config: AIModelConfig): BaseChatModel {
  return new ChatOpenAI({
    openAIApiKey: config.apiKey,
    modelName: config.modelName || "gpt-3.5-turbo",
    temperature: config.temperature ?? 0.7,
    maxTokens: config.maxTokens,
    streaming: config.streaming ?? false,
    ...config.additionalKwargs,
  });
}
