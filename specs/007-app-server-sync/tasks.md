# 태스크: 앱-서버 아티클 동기화

**입력**: `/specs/007-app-server-sync/`의 설계 문서
**선행 조건**: plan.md, spec.md

## 형식: `[ID] [P?] [Story] 설명`

- **[P]**: 병렬 진행 가능 (서로 다른 파일, 의존성 없음)
- **[Story]**: 이 태스크가 속한 사용자 스토리 (예: US1, US2, US3)

---

## 1단계: 설정 (공유 인프라)

**목적**: 프로젝트 환경 설정 및 기본 구조 준비

- [X] T001 `apps/mobile/src-tauri/.env.example` 파일에 `BRAIN_SERVER_URL=http://localhost:3000` 환경변수 추가
- [X] T002 `apps/mobile/src-tauri/src/features/sync/mod.rs` 파일 생성 - 모듈 선언
- [X] T003 `apps/mobile/src-tauri/src/features/mod.rs` 파일에 `pub mod sync;` 추가

---

## 2단계: 기반 (블로킹 선행 조건)

**목적**: 어떠한 사용자 스토리도 구현되기 전에 반드시 완료되어야 하는 핵심 인프라

**⚠️ 중요**: 이 단계가 완료될 때까지 사용자 스토리 작업을 시작할 수 없습니다.

- [X] T004 `apps/mobile/src-tauri/src/features/sync/client.rs` 파일 생성 - BrainServerClient 구조체 정의
- [X] T005 `apps/mobile/src-tauri/src/features/sync/client.rs`에 `new()`, `create_article()` 메서드 구현
- [X] T006 `apps/mobile/src-tauri/src/features/sync/client.rs`에 타임아웃 5초 설정 및 에러 처리 구현
- [X] T007 `apps/mobile/src-tauri/src/features/recommendation/repository.rs`에 `synced_at`, `server_article_id` 컬럼 추가 (SQLite 마이그레이션)
- [X] T008 `apps/mobile/src-tauri/src/features/recommendation/repository.rs`에 `mark_as_synced()`, `is_synced()` 메서드 추가

**체크포인트**: 기반 준비 완료 - 이제 사용자 스토리 구현을 시작할 수 있습니다.

---

## 3단계: 사용자 스토리 1 - 아티클 읽음 시 서버 전송 (우선순위: P1) 🎯 MVP

**목표**: 아티클 상세 보기 시 서버에 아티클 정보 자동 전송

**독립적 테스트**: 앱에서 아티클 클릭 후 서버 `/articles` 목록에 해당 아티클 존재 확인

### 사용자 스토리 1 구현

- [X] T009 [US1] `apps/mobile/src-tauri/src/features/sync/service.rs` 파일 생성 - SyncService 구조체 정의
- [X] T010 [US1] `apps/mobile/src-tauri/src/features/sync/service.rs`에 `sync_article()` 메서드 구현 (중복 체크 포함)
- [X] T011 [US1] `apps/mobile/src-tauri/src/features/sync/mod.rs`에 `sync_article_to_server` Tauri 명령어 추가
- [X] T012 [US1] `apps/mobile/src-tauri/src/lib.rs`에 `sync_article_to_server` 명령어 등록
- [X] T013 [US1] 프론트엔드에서 아티클 상세 보기 시 `sync_article_to_server` 호출 추가 (`apps/web/src/components/ArticleCard.tsx`)
- [X] T014 [US1] 아티클 클릭 → 서버 전송 수동 테스트 및 검증

**체크포인트**: 아티클 클릭 시 서버에 데이터가 전송되고 중복 전송이 방지됨

---

## 4단계: 사용자 스토리 2 - 피드백 서버 전송 (우선순위: P2)

**목표**: 앱에서 피드백 제출 시 서버에도 동시 전송

**독립적 테스트**: 피드백 제출 후 서버 `/articles/:id/feedback` 통계에서 피드백 수 증가 확인

### 사용자 스토리 2 구현

- [X] T015 [US2] `apps/mobile/src-tauri/src/features/sync/client.rs`에 `submit_feedback()` 메서드 추가
- [X] T016 [US2] `apps/mobile/src-tauri/src/features/recommendation/commands.rs`의 `submit_feedback` 함수에 서버 전송 로직 추가
- [X] T017 [US2] 피드백 제출 → 서버 전송 수동 테스트 및 검증

**체크포인트**: 피드백 제출 시 서버에 전송되고 로컬 저장도 유지됨

---

## 5단계: 사용자 스토리 3 - 서버 연결 설정 (우선순위: P1)

**목표**: 서버 URL 설정 및 연결 상태 확인

**독립적 테스트**: `check_server_health` 명령어로 서버 연결 상태 확인

### 사용자 스토리 3 구현

- [X] T018 [US3] `apps/mobile/src-tauri/src/features/sync/client.rs`에 `health_check()` 메서드 추가
- [X] T019 [US3] `apps/mobile/src-tauri/src/features/sync/mod.rs`에 `check_server_health` Tauri 명령어 추가
- [X] T020 [US3] `apps/mobile/src-tauri/src/lib.rs`에 `check_server_health` 명령어 등록
- [X] T021 [US3] 환경변수 변경 후 서버 연결 수동 테스트 및 검증

**체크포인트**: BRAIN_SERVER_URL 환경변수로 서버 URL 변경 가능

---

## 6단계: 마무리 및 공통 관심사

**목적**: 여러 사용자 스토리에 영향을 미치는 개선 사항

- [X] T022 [P] `specs/007-app-server-sync/quickstart.md` 문서 생성
- [X] T023 [P] `apps/mobile/src-tauri/README.md` 파일 업데이트 (서버 연동 설명 추가)
- [X] T024 `cargo test` 실행하여 기존 테스트 통과 확인
- [X] T025 모든 기능 통합 테스트 수행 (앱 → 서버 → /query)
- [X] T026 `GEMINI.md` 파일에 본 기능 추가 기록

---

## 의존성 및 실행 순서

### 단계 의존성

- **설정 (1단계)**: 의존성 없음 - 즉시 시작 가능
- **기반 (2단계)**: 설정 완료에 의존 - 모든 사용자 스토리를 차단함
- **사용자 스토리 1 (3단계)**: 기반 단계 완료에 의존 - MVP
- **사용자 스토리 2 (4단계)**: US1 완료에 의존 (아티클이 전송되어야 피드백 전송 가능)
- **사용자 스토리 3 (5단계)**: 기반 단계 완료에 의존 - US1과 독립적
- **마무리 (6단계)**: 모든 사용자 스토리 완료에 의존

### 사용자 스토리 의존성

```
        ┌─────────────┐
        │   설정 (1)   │
        └──────┬──────┘
               │
        ┌──────▼──────┐
        │   기반 (2)   │
        └──────┬──────┘
               │
     ┌─────────┼─────────┐
     ▼         │         ▼
┌────────┐     │    ┌────────┐
│ US1(P1)│     │    │ US3(P1)│
│ 아티클 │     │    │ 헬스   │
│ 전송   │     │    │ 체크   │
└────┬───┘     │    └────────┘
     │         │
     ▼         │
┌────────┐     │
│ US2(P2)│ ◄───┘
│ 피드백 │
└────────┘
     │
     ▼
┌────────┐
│ 마무리 │
└────────┘
```

### 병렬 기회

- **2단계**: T004~T006 (client.rs)와 T007~T008 (repository.rs) 병렬 가능
- **3~5단계**: US1, US3은 병렬 실행 가능 (US2는 US1 완료 후)
- **6단계**: T022, T023 병렬 실행 가능

---

## 구현 전략

### MVP 우선 (사용자 스토리 1 + 3)

1. **1단계**: 설정 완료
2. **2단계**: 기반 완료
3. **3단계**: 사용자 스토리 1 완료 (아티클 전송)
4. **5단계**: 사용자 스토리 3 완료 (헬스체크)
5. **중지 및 검증**: 아티클 클릭 시 서버에 전송되는지 확인
6. 준비되면 US2 진행

### 점진적 전달

1. 설정 + 기반 완료 → 기반 준비됨
2. 사용자 스토리 1 + 3 추가 → 독립적 테스트 → **MVP 완료**
3. 사용자 스토리 2 추가 → 피드백 동기화 추가
4. 각 스토리는 이전 스토리를 깨뜨리지 않고 가치를 더함

---

## 참고

- [P] 태스크 = 다른 파일, 의존성 없음
- [Story] 라벨은 추적 가능성을 위해 태스크를 특정 사용자 스토리에 매핑함
- 각 사용자 스토리는 독립적으로 완료 가능하고 테스트 가능해야 함
- 각 태스크 또는 논리적 그룹 후 커밋
- 스토리 독립적 검증을 위해 각 체크포인트에서 중지
- 기존 `reqwest::Client`는 `RecommendationState`에서 관리하므로 재사용 권장
