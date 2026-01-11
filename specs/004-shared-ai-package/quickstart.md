# Quickstart: Shared AI Package

이 문서는 `packages/ai`를 프로젝트에 추가하고 사용하는 방법을 설명합니다.

## 설치

워크스페이스 내의 다른 앱(예: `apps/server`)에서 이 패키지를 의존성으로 추가합니다.

```bash
# apps/server 디렉토리에서
pnpm add @z-project/ai --workspace
```

## 사용법

### 1. 기본 텍스트 생성

```typescript
import { createChatModel } from '@z-project/ai';
import { HumanMessage } from '@langchain/core/messages';

const model = createChatModel({
  provider: 'openai', // 또는 'gemini'
  temperature: 0.7,
  // apiKey: process.env.OPENAI_API_KEY // 생략 시 환경변수 확인
});

const response = await model.invoke([
  new HumanMessage("안녕하세요, 오늘 날씨가 어떤가요?")
]);

console.log(response.content);
```

### 2. 환경 변수 설정

각 공급자별로 필요한 환경 변수를 `.env` 파일에 설정해야 합니다.

```bash
# .env
OPENAI_API_KEY=sk-abc...
GEMINI_API_KEY=AIza...
```

### 3. 스트리밍 사용

```typescript
const model = createChatModel({
  provider: 'gemini',
  streaming: true
});

const stream = await model.stream([
  new HumanMessage("이야기 하나 들려줘")
]);

for await (const chunk of stream) {
  process.stdout.write(chunk.content);
}
```
