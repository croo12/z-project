# 태스크: Migrate Server to Gemini

**입력**: `specs/005-migrate-server-to-gemini`의 설계 문서
**선행 조건**: plan.md, spec.md

## 형식: `[ID] [P?] [Story?] 설명`

- **[P]**: 병렬 진행 가능
- **[Story]**: 해당 태스크가 속한 사용자 스토리 (US1, US2)

---

## 1단계: 설정 (환경 구성)

**목적**: Gemini API 키 설정 및 환경 변수 준비.

- [X] T001 `apps/server/.env` 파일에 `GEMINI_API_KEY` 환경 변수 추가 및 타입 정의 업데이트(필요 시).
- [X] T002 `apps/server/src/core/graph.ts`에서 환경 변수 로드 확인 로직 점검.

---

## 2단계: 사용자 스토리 1 (P1) - Gemini로 AI 모델 교체

**목표**: 서버의 AI 모델을 Gemini로 변경하고 정상 동작 확인.
**독립적 테스트**: `/query` 호출 시 Gemini 모델이 응답하는지 확인.

- [X] T003 [US1] `apps/server/src/core/graph.ts`의 `createChatModel` 호출 설정을 provider: 'gemini'로 변경하고 모델명 업데이트.
- [X] T004 [US1] 서버를 실행하여 기본 `/query` 엔드포인트가 Gemini를 통해 정상 응답하는지 수동 검증.

---

## 3단계: 사용자 스토리 2 (P1) - 타입 안정성 확보

**목표**: `graph.ts`의 TypeScript 컴파일 에러 해결.
**독립적 테스트**: `pnpm run build` 실행 시 에러 없음.

- [X] T005 [US2] `apps/server/src/core/graph.ts`의 `RAGGraph` 클래스 및 `this.llm` 속성에 대한 제네릭 타입(`CallOptions`, `OutputMessageType`)을 명시적으로 선언하거나 `BaseChatModel` 타입을 조정하여 컴파일 에러 해결.
- [X] T006 [US2] `apps/server` 전체 빌드(`pnpm run build`)를 실행하여 모든 타입 에러가 제거되었는지 검증.

---

## 4단계: 마무리 및 검증

**목적**: 전체 시스템 안정성 확인.

- [X] T007 `pnpm --filter @z-project/server test`를 실행하여 기존 테스트 스위트가 회귀 없이 통과하는지 확인.
- [X] T008 `quickstart.md` (만약 있다면) 등 문서에 Gemini 설정 관련 내용 업데이트.

---

## 의존성 및 실행 순서

- **1단계 (설정)** 완료 후 2단계 및 3단계 진행.
- **2단계 (모델 교체)**와 **3단계 (타입 수정)**은 코드 수정 위치가 같으므로 순차적(`2 -> 3` 또는 `3 -> 2`)으로 진행하거나 동시에 작업(코드 수정이 겹침).
- **4단계 (마무리)**는 모든 수정 완료 후 실행.

## 구현 전략

### 통합 접근

`graph.ts` 파일 하나에서 모델 교체와 타입 수정이 동시에 일어나야 하므로, 하나의 작업 흐름으로 처리하는 것이 효율적입니다.

1.  환경 변수 설정 (T001).
2.  코드 수정 (T003, T005 동시 진행).
3.  빌드 및 테스트 검증 (T006, T007).
