# 연구 결과: Shared AI Package

**날짜**: 2026-01-11
**상태**: 완료

## 결정 사항

### 1. 기술 스택: LangChain.js
- **결정**: AI 모델 인터페이스로 `@langchain/core`, `@langchain/openai`, `@langchain/google-genai`를 사용한다.
- **근거**:
    - `apps/server`에서 이미 LangChain을 사용 중이므로 호환성이 좋다.
    - `BaseChatModel`이라는 강력한 추상화 계층을 제공하여, 코드 수정 없이 모델 교체가 용이하다.
    - 스트리밍, 도구 호출 등 고급 기능을 표준화된 방식으로 지원한다.
- **대안**:
    - **공식 SDK 직접 사용**: OpenAI SDK와 Gemini SDK를 직접 래핑. -> 장점은 가볍지만, 인터페이스 통일을 위해 많은 보일러플레이트 코드를 작성해야 함.
    - **Vercel AI SDK**: Next.js 중심이나 Node.js에서도 사용 가능. -> LangChain이 이미 서버에서 사용되고 있어 일관성 유지가 더 중요함.

### 2. 패키지 구조: pnpm workspace
- **결정**: `packages/ai` 경로에 독립적인 npm 패키지로 생성한다.
- **근거**: 모노레포 아키텍처를 따르며, 코드 재사용성을 극대화한다.

## 미해결 질문 (해결됨)

- **Gemini 라이브러리**: `@google/generative-ai` vs `@langchain/google-genai`
    - **답변**: LangChain 생태계 통합을 위해 `@langchain/google-genai`를 선택한다.
