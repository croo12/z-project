# 데이터 모델: Shared AI Package

이 문서는 `packages/ai`에서 노출하는 주요 타입과 인터페이스를 정의합니다.

## 공통 타입

### AIProvider
지원하는 AI 제공자 목록입니다.

```typescript
export type AIProvider = 'openai' | 'gemini';
```

### AIModelConfig
모델 생성 시 전달하는 설정 객체입니다.

```typescript
export interface AIModelConfig {
  /**
   * 사용할 AI 공급자
   */
  provider: AIProvider;

  /**
   * API 키 (생략 시 환경 변수 사용)
   * OpenAI: OPENAI_API_KEY
   * Gemini: GOOGLE_API_KEY
   */
  apiKey?: string;

  /**
   * 사용할 모델명 (예: 'gpt-4o', 'gemini-1.5-pro')
   * 생략 시 공급자별 기본값 사용
   */
  modelName?: string;

  /**
   * 생성 온도 (0.0 ~ 1.0)
   * default: 0.7
   */
  temperature?: number;

  /**
   * 최대 생성 토큰 수
   */
  maxTokens?: number;

  /**
   * 스트리밍 활성화 여부
   * default: false
   */
  streaming?: boolean;

  /**
   * 추가 공급자별 옵션
   */
  additionalKwargs?: Record<string, any>;
}
```

## 팩토리 인터페이스

### createChatModel
설정에 따라 LangChain의 `BaseChatModel` 인스턴스를 반환하는 팩토리 함수입니다.

```typescript
import { BaseChatModel } from "@langchain/core/language_models/chat_models";

export function createChatModel(config: AIModelConfig): BaseChatModel;
```
