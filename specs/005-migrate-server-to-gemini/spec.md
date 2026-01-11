# 기능 명세서: Migrate Server to Gemini

**기능 브랜치**: `005-migrate-server-to-gemini`
**생성일**: 2026-01-11
**상태**: 초안
**입력**: "apps/server의 ai를 gemini로 교체하고 지금 있는 타입 에러도 제거하려합니다."

## 사용자 시나리오 및 테스트 *(필수)*

### 사용자 스토리 1 - Gemini로 AI 모델 교체 (우선순위: P1)

개발자로서, 나는 `apps/server`가 OpenAI 대신 Google Gemini 모델을 사용하여 비용 효율적이고 성능이 좋은 AI 응답을 생성하기를 원한다. 이를 위해 `apps/server`는 이전에 생성된 `packages/ai`를 사용하여 Gemini 모델을 호출해야 한다.

**이 우선순위인 이유**: 현재 시스템의 기본 AI 제공자를 Gemini로 변경하여 운영 효율성을 높이려 함.

**독립적 테스트**: 서버의 `/query` 엔드포인트를 호출했을 때 에러 없이 응답이 오는지 확인하고, 로그 상에서 Gemini Provider가 사용되었는지 확인한다.

**인수 시나리오**:

1.  **Given** 서버가 실행 중이고 `GEMINI_API_KEY`가 설정되어 있을 때, **When** `/query` 엔드포인트에 질문을 보내면, **Then** Gemini 모델이 생성한 답변을 받아야 한다.
2.  **Given** `packages/ai`가 올바르게 연동되어 있을 때, **When** 서버 코드를 검사하면, **Then** `ChatOpenAI` 직접 호출이 아닌 `createChatModel({ provider: 'gemini' })`가 사용되어야 한다.

### 사용자 스토리 2 - 타입 안정성 확보 (우선순위: P1)

개발자로서, 나는 `apps/server`의 TypeScript 컴파일 에러(특히 제네릭 타입 관련 에러)를 제거하여 코드의 안정성을 확보하고 빌드가 깨지지 않게 하고 싶다.

**이 우선순위인 이유**: 현재 `graph.ts` 등에서 제네릭 타입 불일치로 인한 에러가 발생하고 있어 CI/CD 및 배포에 문제가 될 수 있음.

**독립적 테스트**: `apps/server` 디렉토리에서 `pnpm run build` 또는 `tsc`를 실행했을 때 에러 없이 컴파일이 완료되어야 한다.

**인수 시나리오**:

1.  **Given** `graph.ts` 파일이 수정되었을 때, **When** `pnpm run build`를 실행하면, **Then** 타입 에러 없이 성공해야 한다.

## 요구사항 *(필수)*

### 기능적 요구사항

- **FR-001**: `apps/server/src/core/graph.ts`에서 `ChatOpenAI`를 제거하고 `@z-project/ai`의 `createChatModel`을 사용하여 Gemini 모델로 초기화해야 한다.
- **FR-002**: `apps/server` 실행 시 필요한 환경 변수(`GEMINI_API_KEY`)를 `.env` 파일에 추가하고 로드할 수 있어야 한다.

### 비기능적 요구사항

- **NFR-001**: `BaseChatModel`의 제네릭 타입 파라미터(`CallOptions`, `OutputMessageType`)를 명시하거나 적절히 처리하여 TypeScript 컴파일 에러를 해결해야 한다.
- **NFR-002**: 기존 테스트(`apps/server/src/index.test.ts` 등)가 변경된 로직에서도 통과해야 한다.

## 성공 기준 *(필수)*

### 측정 가능한 결과

- **SC-001**: `apps/server`의 빌드가 에러 없이 성공해야 한다 (`tsc` 통과).
- **SC-002**: 서버가 시작되고 `/query` 요청에 정상 응답해야 한다.
- **SC-003**: `pnpm --filter @z-project/server test` 명령어 실행 시 모든 테스트가 통과해야 한다.
