# @z-project/ai

OpenAI 및 Google Gemini와 같은 AI 모델을 통합적으로 사용하기 위한 공유 패키지입니다.

## 설치

```bash
pnpm add @z-project/ai --workspace
```

## 사용법

### 모델 생성

```typescript
import { createChatModel } from '@z-project/ai';

const model = createChatModel({
  provider: 'openai', // or 'gemini'
  apiKey: '...', // Optional via env
  modelName: 'gpt-4',
  temperature: 0.7
});

const response = await model.invoke("Hello!");
```

### 새 제공자 추가 (Extensibility)

기본적으로 `openai`와 `gemini`를 지원합니다. 새로운 제공자를 런타임에 추가할 수 있습니다.

```typescript
import { registerProvider } from '@z-project/ai';

registerProvider('my-custom-llm', (config) => {
  return new MyCustomLLM(config);
});

const model = createChatModel({ provider: 'my-custom-llm' as any });
```

## 테스트

```bash
pnpm --filter @z-project/ai test
```
