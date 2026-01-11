# 구현 계획: Shared AI Package

**브랜치**: `004-shared-ai-package` | **날짜**: 2026-01-11 | **명세서**: [spec.md](spec.md)
**입력**: `specs/004-shared-ai-package/spec.md`의 기능 명세

**참고**: 이 템플릿은 `/speckit.plan` 명령에 의해 작성됩니다. 실행 워크플로우에 대해서는 `.specify/templates/commands/plan.md`를 참조하세요.

## 요약

이 프로젝트는 모노레포 내에서 AI 모델(OpenAI, Google Gemini)과의 상호작용을 중앙 집중화하고 표준화하기 위해 `packages/ai`라는 공유 패키지를 신설합니다. 이 패키지는 LangChain.js를 기반으로 하여 일관된 `BaseChatModel` 인터페이스를 제공하며, 애플리케이션의 다른 부분(예: `apps/server`)에서 쉽게 AI 기능을 사용할 수 있도록 합니다.

## 기술적 맥락

**언어/버전**: TypeScript (Node.js v20+)
**주요 의존성**: 
- `@langchain/core` (공통 인터페이스)
- `@langchain/openai` (OpenAI 구현체)
- `@langchain/google-genai` (Google Gemini 구현체)
**저장소**: 해당 없음 (Stateless 라이브러리)
**테스트**: Vitest (단위 및 통합 테스트)
**대상 플랫폼**: Node.js (서버 및 CLI 환경)
**프로젝트 유형**: 라이브러리 (Shared Package)
**성능 목표**: 오버헤드 최소화 (직접 SDK 호출과 유사한 성능)
**제약 사항**: API 키는 패키지 내부가 아닌 호출자가 주입하거나 환경 변수를 통해 관리해야 함.
**규모/범위**: 초기에는 OpenAI와 Gemini 지원, 추후 확장 가능.

## 헌법(Constitution) 체크

*GATE: Phase 0 조사 전 통과 필수. Phase 1 설계 후 재확인.*

- **1. 규칙 기반 AI 개발**: LangChain을 사용하여 표준화된 인터페이스를 따르며, 일관된 코딩 스타일을 유지합니다.
- **2. 구성 가능한 프로젝트 명세**: AI 로직을 독립적인 패키지로 분리하여 다른 프로젝트(앱)에서 재사용 가능하게 함으로써 구성 가능성을 높입니다.
- **3. 자동화 및 검증 가능한 워크플로우**: 이 계획은 테스트(Vitest)와 린트(ESLint) 설정을 포함하여 자동화된 품질 관리를 보장합니다.
- **4. 구조화 및 프로세스 지향 태스크 실행**: Spec-Kit 워크플로우를 따라 계획, 설계, 구현 단계를 거칩니다.
- **5. 한국어 우선 문서화**: 모든 문서(spec, plan, tasks)는 한국어로 작성됩니다.

**결과**: 모든 게이트 통과.

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/004-shared-ai-package/
├── plan.md              # 본 파일
├── research.md          # 0단계 출력 (해당 사항 없음, 기술 선택 완료)
├── data-model.md        # 1단계 출력
├── quickstart.md        # 1단계 출력
├── contracts/           # 1단계 출력 (Interfaces)
└── tasks.md             # 2단계 출력
```

### 소스 코드 (레포지토리 루트)

```text
packages/
└── ai/
    ├── src/
    │   ├── factories/   # 모델 생성 팩토리
    │   ├── providers/   # 각 공급자별 설정 및 래퍼
    |   ├── types/       # 공통 타입 정의
    │   └── index.ts     # 진입점
    ├── tests/
    │   ├── integration/
    │   └── unit/
    ├── package.json
    ├── tsconfig.json
    └── vitest.config.ts
```

**구조 결정**: `packages/ai`에 위치하여 모노레포의 `workspace:*` 프로토콜을 이용해 다른 앱에서 참조할 수 있도록 합니다.

## 복잡도 추적

> **헌법 체크에 정당화가 필요한 위반 사항이 있는 경우에만 작성하세요**

| 위반 사항 | 필요한 이유 | 거부된 더 단순한 대안 |
|-----------|-------------|-----------------------|
| *없음*      | -          | -                     |
