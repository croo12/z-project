# 태스크: AI Brain Server

**입력**: `specs/001-ai-brain-server`의 설계 문서
**선행 조건**: plan.md, spec.md, research.md, data-model.md, contracts/

## 형식: `[ID] [P?] [Story?] 설명`

- **[P]**: 병렬 진행 가능
- **[Story]**: 해당 태스크가 속한 사용자 스토리 (US1, US2, US3)

---

## 1단계: 설정 (공유 인프라)

**목적**: `apps/server`를 위한 프로젝트 초기화 및 기본 도구 설정.

- [X] T001 `apps/server/package.json`에 새로운 Node.js 프로젝트 초기화.
- [X] T002 [P] `apps/server/`에 코드 품질을 위한 ESLint 및 Prettier 추가 및 구성.
- [X] T003 [P] `apps/server/vitest.config.ts`에 테스트를 위한 Vitest 추가 및 구성.
- [X] T004 [P] N-API 모듈을 위해 `apps/server/rust_components/`에 새로운 Rust 라이브러리 프로젝트 초기화.
- [X] T005 [P] `apps/server/rust_components/Cargo.toml`에 `napi-rs`를 의존성으로 추가.

---

## 2단계: 기반 (블로킹 선행 조건)

**목적**: 사용자 스토리를 구현하기 전에 반드시 완료해야 하는 핵심 인프라.

- [X] T006 `apps/server/src/index.ts`에 기본 Express.js 서버 설정.
- [X] T007 `apps/server/src/types/index.ts`의 데이터 모델을 기반으로 핵심 데이터 타입(Interaction, Feedback) 정의.
- [X] T008 [P] 인덱스 로드 및 저장을 담당하는 FAISS 벡터 스토어 래퍼 서비스 `apps/server/src/lib/vector-store.ts` 생성.
- [X] T009 [P] `apps/server/src/core/graph.ts`에 LangGraph 그래프의 기본 구조 생성.
- [X] T010 Node.js에 Rust 함수를 노출하기 위해 `apps/server/rust_components/src/lib.rs` 및 `apps/server/src/lib/rust-addon.ts`에 N-API 인터페이스 정의.

---

## 3단계: 사용자 스토리 2 (P1) - 실시간 지식 수집

**목표**: 지식 베이스에 실시간으로 새로운 정보를 추가하는 기능 구현.
**독립적 테스트**: `/knowledge` 엔드포인트에 문서를 전송한 후, 후속 `/query` 호출이 해당 정보를 검색할 수 있는지 검증.

- [X] T011 [US2] `apps/server/src/api/knowledge.ts`에 `POST /knowledge` 지식 수집 API 엔드포인트 생성.
- [X] T012 [US2] 텍스트 분할, 임베딩 생성, 래퍼를 통해 FAISS 저장소에 문서 추가를 처리하는 `IngestionService`를 `apps/server/src/services/ingestion.service.ts`에 구현.

---

## 4단계: 사용자 스토리 1 (P1) - 맥락 기반 인터랙션 및 피드백

**목표**: 사용자-AI 상호작용을 기록하고 이에 대한 피드백을 받는 기능 구현.
**독립적 테스트**: `/interactions` 엔드포인트를 호출하여 ID를 받고, 해당 ID로 `/feedback` 엔드포인트를 호출하여 데이터가 저장되는지 검증.

- [X] T013 [P] [US1] `apps/server/src/api/interactions.ts`에 `POST /interactions` 상호작용 로깅 API 엔드포인트 생성.
- [X] T014 [P] [US1] `apps/server/src/api/feedback.ts`에 `POST /feedback` 피드백 제출 API 엔드포인트 생성.
- [X] T015 [US1] `apps/server/src/services/feedback.service.ts`에 `FeedbackService` 구현. 초기에 이 서비스는 상호작용 및 피드백 데이터만 저장함. (점수 조정 로직은 US3에 포함).

---

## 5단계: 사용자 스토리 3 (P2) - 향상된 AI 응답

**목표**: 저장된 지식과 피드백을 사용하여 사용자 쿼리에 보강된 응답 제공.
**독립적 테스트**: 벡터 저장소에 알려진 고유 정보가 있는 상태에서, 관련 질문으로 `/query`를 호출하고 응답에 해당 고유 정보가 있는지 검증.

- [X] T016 [US3] FAISS로부터의 검색을 포함하여 `apps/server/src/core/graph.ts`의 LangGraph 그래프 내에 핵심 RAG 로직 구현.
- [X] T017 [US3] `apps/server/src/lib/vector-store.ts`를 FAISS 대신 LanceDB를 사용하도록 마이그레이션하고, `FeedbackService`에 `retrieval_score_modifier` 업데이트 로직 구현.
- [X] T018 [US3] 응답을 얻기 위해 LangGraph를 호출하는 쿼리 API 엔드포인트 `POST /query`를 `apps/server/src/api/query.ts`에 생성.

---

## 6단계: 마무리 및 공통 관심사

**목적**: 전체 서비스에 영향을 미치는 개선 사항.

- [X] T019 [P] 애플리케이션 전체에 구조화된 로깅(예: Pino 또는 Winston 사용) 구현.
- [X] T020 [P] 서버 포트 및 데이터베이스 경로와 같은 설정을 위한 환경 변수 구성(예: `dotenv` 사용) 구현.
- [X] T021 배포를 위해 애플리케이션을 컨테이너화하기 위한 `apps/server`용 `Dockerfile` 생성.
- [X] T022 완전하고 검증된 설정 및 실행 지침으로 `quickstart.md` 마무리.

---

## 의존성 및 실행 순서

- **1단계 (설정)**가 먼저 완료되어야 함.
- **2단계 (기반)**는 1단계에 의존함.
- **3단계 (US2)**와 **4단계 (US1)**는 2단계 완료 후 병렬로 시작 가능함.
- **5단계 (US3)**는 3단계 (지식 수집) 및 4단계 (피드백 메커니즘) 완료에 의존함.
- **6단계 (마무리)**는 마지막에 수행 가능함.

## 구현 전략

### MVP 우선 (사용자 스토리 1 & 2)

1.  1단계 & 2단계 완료.
2.  3단계 (지식 수집) 완료.
3.  4단계 (상호작용 & 피드백) 완료.
4.  **중지 및 검증**: 이 시점에서 시스템은 학습하고 피드백을 받을 수 있으며, 이는 실행 가능한 MVP를 형성함. 핵심 데이터 루프가 완료됨.

### 전체 기능

1.  MVP 단계 완료.
2.  수집된 데이터를 활용하기 위해 5단계 (향상된 응답) 완료.
3.  운영 준비를 위해 6단계 (마무리) 완료.
