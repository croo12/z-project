import { describe, it, expect } from 'vitest';
import { createChatModel, registerProvider } from '../../src/factories/chat-model.js';
import { BaseChatModel } from "@langchain/core/language_models/chat_models";
import { AIModelConfig } from '../../src/types/index.js';
import { AIMessage } from "@langchain/core/messages";

// Mock implementation of a ChatModel
class MockChatModel extends BaseChatModel {
  _llmType() { return "mock"; }
  async _generate(messages: any[], options: any) {
    return { generations: [{ text: "Mock response", message: new AIMessage("Mock response") }] };
  }
}

describe('AI Extensibility', () => {
  it('should allow registering a new provider', () => {
    const mockFactory = (config: AIModelConfig) => new MockChatModel({});
    
    // Register new provider
    registerProvider('mock-provider', mockFactory);
    
    // Create instance using new provider
    const model = createChatModel({
      provider: 'mock-provider' as any, // Cast to any since type is strict
    });
    
    expect(model).toBeInstanceOf(MockChatModel);
    expect(model._llmType()).toBe('mock');
  });

  it('should still throw for unregistered provider', () => {
    expect(() => {
      createChatModel({
        provider: 'unknown-provider' as any,
      });
    }).toThrow('Unsupported AI provider');
  });
});
