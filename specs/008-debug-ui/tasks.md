# 태스크: 디버그 모드 UI 개선

**입력**: `/specs/008-debug-ui/`의 설계 문서
**선행 조건**: plan.md, spec.md, research.md

## 형식: `[ID] [P?] [Story] 설명`

- **[P]**: 병렬 진행 가능 (서로 다른 파일, 의존성 없음)
- **[Story]**: 이 태스크가 속한 사용자 스토리 (예: US1, US2, US3)

---

## 1단계: 기반 (블로킹 선행 조건)

**목적**: 디버그 기능 구현을 위한 백엔드 모듈 구조 및 프론트엔드 기반 마련

- [ ] T001 `apps/mobile/src-tauri/src/features/debug/mod.rs` 파일 생성 (디버그 모듈 정의)
- [ ] T002 `apps/mobile/src-tauri/src/features/mod.rs`에 debug 모듈 등록 로직 추가 (`#[cfg(debug_assertions)]` 조건부 컴파일)
- [ ] T003 `apps/mobile/src-tauri/src/lib.rs`에 debug 모듈 연동 (조건부 컴파일)
- [ ] T004 `apps/web/src/components/Debug/DebugOverlay.tsx` 생성 (빈 껍데기 컴포넌트)

---

## 2단계: 사용자 스토리 1 - 디버그 오버레이 확인 (우선순위: P1)

**목표**: 디버그 빌드에서만 화면 상단에 열고 닫을 수 있는 오버레이 UI 표시

**독립적 테스트**: `pnpm dev` 실행 시 오버레이가 보이고 접고 펼칠 수 있어야 함

- [ ] T005 [P] [US1] `apps/web/src/components/Debug/DebugOverlay.tsx`에 접기/펼치기 상태 및 기본 스타일(Fixed Top) 구현
- [ ] T006 [US1] `apps/web/src/App.tsx`에 `import.meta.env.DEV` 체크를 통한 `DebugOverlay` 조건부 렌더링 추가
- [ ] T007 [US1] 오버레이 표시 여부 및 토글 기능 수동 테스트

---

## 3단계: 사용자 스토리 2 - 서버 연결 상태 확인 (우선순위: P1)

**목표**: 오버레이 내에서 서버 URL을 확인하고 연결 상태(Healthy/Unhealthy)를 실시간 표시

**독립적 테스트**: 서버를 끄고 켰을 때 오버레이의 상태 표시가 변경되어야 함

- [ ] T008 [P] [US2] `apps/web/src/components/Debug/ServerStatus.tsx` 컴포넌트 생성 및 `BRAIN_SERVER_URL` 표시 로직 구현
- [ ] T009 [US2] `apps/web/src/components/Debug/ServerStatus.tsx`에 `check_server_health` Tauri 커맨드 주기적 호출(interval) 구현
- [ ] T010 [US2] `apps/web/src/components/Debug/DebugOverlay.tsx`에 `ServerStatus` 컴포넌트 포함
- [ ] T011 [US2] 서버 연결 상태 표시(초록/빨강) 및 자동 갱신 수동 테스트

---

## 4단계: 사용자 스토리 3 - AI 동작 테스트 (우선순위: P2)

**목표**: 오버레이에서 AI 기능을 테스트하고 결과를 확인

**독립적 테스트**: "Test AI" 버튼 클릭 시 팝업으로 결과가 표시되어야 함

- [ ] T012 [US3] `apps/mobile/src-tauri/src/features/debug/commands.rs` 생성 및 `test_ai_connection` 커맨드 구현 (Gemini API 호출 또는 더미 응답)
- [ ] T013 [US3] `apps/mobile/src-tauri/src/features/debug/mod.rs`에 `test_ai_connection` 커맨드 등록
- [ ] T014 [P] [US3] `apps/web/src/components/Debug/AITester.tsx` 컴포넌트 생성 ("Test AI" 버튼 및 핸들러)
- [ ] T015 [US3] `apps/web/src/components/Debug/AITester.tsx`에 `test_ai_connection` 호출 및 결과 표시(alert/modal) 로직 구현
- [ ] T016 [US3] `apps/web/src/components/Debug/DebugOverlay.tsx`에 `AITester` 컴포넌트 포함
- [ ] T017 [US3] AI 테스트 버튼 동작 및 결과 표시 수동 테스트

---

## 5단계: 마무리 및 공통 관심사

**목적**: 코드 정리 및 문서화

- [ ] T018 `GEMINI.md` 파일에 디버그 기능 관련 변경 사항 기록
- [ ] T019 `specs/008-debug-ui/quickstart.md` (선택) 또는 관련 문서 업데이트

---

## 의존성 및 실행 순서

### 단계 의존성

- **기반 (1단계)**: 의존성 없음 - 가장 먼저 수행
- **US1 (2단계)**: 기반 완료 후 수행
- **US2 (3단계)**: US1(오버레이) 완료 후 수행 (오버레이 안에 들어가야 함)
- **US3 (4단계)**: US1 완료 및 백엔드 기반 완료 후 수행

### 병렬 기회

- T005, T008, T014 등 React 컴포넌트 작업은 백엔드 작업과 병렬 진행 가능하지만, 통합을 위해 순차 진행 권장
- US2와 US3은 US1 완료 후 서로 병렬로 진행 가능
