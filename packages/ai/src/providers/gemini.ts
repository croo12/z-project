import { ChatGoogleGenerativeAI } from "@langchain/google-genai";
import { BaseChatModel } from "@langchain/core/language_models/chat_models";
import { AIModelConfig } from "../types/index.js";

export function createGeminiModel(config: AIModelConfig): BaseChatModel {
  return new ChatGoogleGenerativeAI({
    apiKey: config.apiKey,
    model: config.modelName || "gemini-pro",
    temperature: config.temperature ?? 0.7,
    maxOutputTokens: config.maxTokens,
    streaming: config.streaming ?? false,
    ...config.additionalKwargs,
  });
}
