# 구현 계획: Migrate Server to Gemini

**브랜치**: `005-migrate-server-to-gemini` | **날짜**: 2026-01-11 | **명세서**: [spec.md](spec.md)
**입력**: `specs/005-migrate-server-to-gemini/spec.md`의 기능 명세

**참고**: 이 템플릿은 `/speckit.plan` 명령에 의해 작성됩니다. 실행 워크플로우에 대해서는 `.specify/templates/commands/plan.md`를 참조하세요.

## 요약

이 프로젝트는 `apps/server`의 AI 제공자를 OpenAI에서 Google Gemini로 마이그레이션하고, 관련된 TypeScript 컴파일 에러를 해결하는 것을 목표로 합니다. `packages/ai` (Shared AI Package)를 활용하여 `apps/server` 코드를 리팩토링함으로써 코드 중복을 줄이고 유지보수성을 향상시킵니다.

## 기술적 맥락

**언어/버전**: TypeScript (Node.js v20+)
**주요 의존성**:
- `@z-project/ai` (workspace shared package)
- `@langchain/core`
- `@langchain/google-genai`
- `@langchain/langgraph`
**저장소**: LanceDB (기존 유지)
**테스트**: Vitest (Unit & Integration)
**대상 플랫폼**: Node.js Server
**프로젝트 유형**: Backend Server Refactoring
**성능 목표**: Gemini 모델을 사용하여 기존과 동등하거나 더 나은 응답 성능 확보
**제약 사항**: 기존 기능(RAG, Graph Workflow)의 동작을 보장해야 함.

## 헌법(Constitution) 체크

*GATE: Phase 0 조사 전 통과 필수. Phase 1 설계 후 재확인.*

- **1. 규칙 기반 AI 개발**: 명확한 스펙과 계획에 따라 체계적으로 마이그레이션을 수행합니다.
- **2. 구성 가능한 프로젝트 명세**: 공유 패키지(`packages/ai`)를 활용하여 앱의 구성을 모듈화합니다.
- **3. 자동화 및 검증 가능한 워크플로우**: 빌드 및 테스트 자동화를 통해 변경 사항을 검증합니다.
- **4. 구조화 및 프로세스 지향 태스크 실행**: Spec-Kit 워크플로우를 준수합니다.
- **5. 한국어 우선 문서화**: 모든 문서를 한국어로 작성합니다.

**결과**: 모든 게이트 통과.

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/005-migrate-server-to-gemini/
├── plan.md              # 본 파일
├── research.md          # 0단계 출력 (해당 사항 없음)
├── data-model.md        # 1단계 출력 (해당 사항 없음, 기존 모델 사용)
├── quickstart.md        # 1단계 출력 (해당 사항 없음, 기존 문서 업데이트)
├── contracts/           # 1단계 출력 (해당 사항 없음)
└── tasks.md             # 2단계 출력
```

### 소스 코드 (레포지토리 루트)

```text
apps/
└── server/
    ├── src/
    │   ├── core/
    │   │   └── graph.ts     # 주요 변경 대상 (AI 호출 로직)
    │   └── lib/
    │       └── vector-store.ts
    └── package.json         # 의존성 관리
```

**구조 결정**: 기존 `apps/server` 구조를 유지하며 코드 리팩토링에 집중합니다.

## 복잡도 추적

> **헌법 체크에 정당화가 필요한 위반 사항이 있는 경우에만 작성하세요**

| 위반 사항 | 필요한 이유 | 거부된 더 단순한 대안 |
|-----------|-------------|-----------------------|
| *없음*      | -          | -                     |
