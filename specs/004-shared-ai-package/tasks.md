# 태스크: Shared AI Package

**입력**: `specs/004-shared-ai-package`의 설계 문서
**선행 조건**: plan.md, spec.md, research.md, data-model.md, contracts/

## 형식: `[ID] [P?] [Story?] 설명`

- **[P]**: 병렬 진행 가능
- **[Story]**: 해당 태스크가 속한 사용자 스토리 (US1, US2)

---

## 1단계: 설정 (공유 인프라)

**목적**: `packages/ai` 패키지 초기화 및 기본 환경 구성.

- [X] T001 `packages/ai/package.json` 생성 및 LangChain 관련 의존성 추가 (@langchain/core, @langchain/openai, @langchain/google-genai 등).
- [X] T002 [P] TypeScript 설정 파일 `packages/ai/tsconfig.json` 생성 (모노레포 기준 준수).
- [X] T003 [P] 테스트 환경 구성을 위한 `packages/ai/vitest.config.ts` 생성.
- [X] T004 [P] ESLint 설정을 위한 `packages/ai/.eslintrc.js` (또는 프로젝트 표준에 맞게) 구성.

---

## 2단계: 기반 (블로킹 선행 조건)

**목적**: 핵심 타입 및 공통 인터페이스 정의.

- [X] T005 `specs/004-shared-ai-package/data-model.md` 및 `contracts/api.yml`을 기반으로 `packages/ai/src/types/index.ts`에 `AIProvider`, `AIModelConfig` 등 타입 정의.

---

## 3단계: 사용자 스토리 1 (P1) - 통합된 AI 클라이언트

**목표**: OpenAI 및 Gemini 모델을 생성하는 통합 팩토리 구현.
**독립적 테스트**: `tests/integration/factory.test.ts`를 실행하여 각 공급자별 모델 생성이 성공하고, 기본 응답이 오는지 확인.

- [X] T006 [US1] `packages/ai/src/providers/openai.ts`에 OpenAI ChatModel 생성 로직 구현.
- [X] T007 [P] [US1] `packages/ai/src/providers/gemini.ts`에 Gemini ChatModel 생성 로직 구현.
- [X] T008 [US1] `packages/ai/src/factories/chat-model.ts`에 `createChatModel` 팩토리 함수 구현 (설정에 따라 적절한 제공자 호출).
- [X] T009 [US1] `packages/ai/src/index.ts`에서 주요 함수 및 타입 export 처리.
- [X] T010 [US1] 각 공급자별 모델 인스턴스화 및 기본 호출을 검증하는 통합 테스트 `packages/ai/tests/integration/factory.test.ts` 작성.

---

## 4단계: 사용자 스토리 2 (P2) - 확장 가능한 구조

**목표**: 새로운 제공자 추가가 용이함을 검증하고 관련 가이드/Mock 테스트 추가.
**독립적 테스트**: Mock Provider를 추가하여 기존 로직 변경 없이 확장 가능한지 테스트.

- [X] T011 [US2] 새로운 제공자 추가 시 확장성을 검증하기 위한 단위 테스트 `packages/ai/tests/unit/extensibility.test.ts` 작성 (Mock Provider 활용).
- [X] T012 [P] [US2] 패키지 사용법 및 기여 방법(새 모델 추가 등)을 설명하는 `packages/ai/README.md` 작성.

---

## 5단계: 마무리 및 공통 관심사

**목적**: 최종 점검 및 배포 준비.

- [X] T013 프로젝트 루트의 `pnpm-workspace.yaml` 설정 재확인 및 전체 빌드/린트 검사 (`pnpm run build`, `pnpm run lint`).
- [X] T014 `quickstart.md` 내용을 바탕으로 최종 사용 가이드 업데이트.

---

## 6단계: 마이그레이션 및 통합 (추가 요청)

**목적**: 기존 애플리케이션(`apps/server`)이 새로운 공유 패키지를 사용하도록 전환.

- [X] T015 `apps/server/package.json`에 `@z-project/ai` 의존성 추가 (workspace 프로토콜 사용).
- [X] T016 `apps/server` 내에서 직접 LangChain/OpenAI를 호출하던 코드를 `packages/ai`의 `createChatModel`을 사용하도록 리팩토링.
- [X] T017 리팩토링 후 `apps/server`의 기존 테스트(`pnpm --filter @z-project/server test`)가 정상 통과하는지 검증.

---

## 의존성 및 실행 순서

- **1단계 (설정)**가 완료되어야 함.
- **2단계 (기반)**는 1단계 이후 진행.
- **3단계 (US1)**는 2단계 완료 후 진행. (T006, T007은 병렬 가능)
- **4단계 (US2)**는 3단계 구조가 잡힌 후 검증 성격으로 진행.
- **5단계 (마무리)**는 모든 구현 완료 후 수행.
- **6단계 (마무리/통합)**는 3단계(패키지 기능 구현) 이후 언제든 진행 가능하지만, 안정성을 위해 5단계 이후 권장.

## 구현 전략

### MVP (US1 + US2 일부)

1.  패키지 설정 및 타입 정의 (T001~T005).
2.  OpenAI 및 Gemini 연동 구현 (T006~T009).
3.  기본 통합 테스트로 동작 확인 (T010).

### 전체 기능 및 통합

1.  확장성 테스트 및 문서화 완료 (T011~T012).
2.  전체 프로젝트 린트/빌드 점검 (T013~T014).
3.  기존 앱 마이그레이션 수행 (T015~T017).
