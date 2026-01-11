import { describe, it, expect } from 'vitest';
import { createChatModel } from '../../src/factories/chat-model.js';
import { ChatOpenAI } from '@langchain/openai';
import { ChatGoogleGenerativeAI } from '@langchain/google-genai';

describe('AI Chat Model Factory', () => {
  it('should create an OpenAI model instance', () => {
    const model = createChatModel({
      provider: 'openai',
      apiKey: 'test-key',
      modelName: 'gpt-4',
    });
    expect(model).toBeInstanceOf(ChatOpenAI);
    // @ts-ignore
    expect(model.modelName).toBe('gpt-4');
  });

  it('should create a Gemini model instance', () => {
    const model = createChatModel({
      provider: 'gemini',
      apiKey: 'test-key',
      modelName: 'gemini-1.5-pro',
    });
    expect(model).toBeInstanceOf(ChatGoogleGenerativeAI);
    // @ts-ignore
    expect(model.modelName).toBe('gemini-1.5-pro');
  });

  it('should throw error for unsupported provider', () => {
    expect(() => {
      createChatModel({
        provider: 'invalid' as any,
      });
    }).toThrow('Unsupported AI provider');
  });
});
