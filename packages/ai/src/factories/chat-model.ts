import { BaseChatModel } from "@langchain/core/language_models/chat_models";
import { AIModelConfig } from "../types/index.js";
import { createOpenAIModel } from "../providers/openai.js";
import { createGeminiModel } from "../providers/gemini.js";

type ModelFactory = (config: AIModelConfig) => BaseChatModel;

const factories: Record<"openai" | "gemini", ModelFactory> = {
  openai: createOpenAIModel,
  gemini: createGeminiModel,
};

export function registerProvider(provider: "openai" | "gemini", factory: ModelFactory) {
  factories[provider] = factory;
}

export function createChatModel(config: AIModelConfig): BaseChatModel {
  const factory = factories[config.provider];
  if (!factory) {
    throw new Error(`Unsupported AI provider: ${config.provider}`);
  }
  return factory(config);
}
