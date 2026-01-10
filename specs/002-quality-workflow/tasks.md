# 태스크: 자동화된 코드 품질 워크플로우

**입력**: `specs/002-quality-workflow/`의 설계 문서
**선행 조건**: plan.md, spec.md

## 형식: `[ID] [P?] [Story?] 설명`

- **[P]**: 병렬 진행 가능
- **[Story]**: 해당 태스크가 속한 사용자 스토리 (US1, US2, US3)

---

## 1단계: 설정 (공유 인프라)

**목적**: git 훅 및 품질 도구 구성을 초기화.

- [x] T001 루트 디렉토리에 Husky 초기화 및 package.json에 prepare 스크립트 추가.
- [x] T002 루트에 lint-staged를 개발 의존성으로 설치.
- [x] T003 [P] 루트에 `.lintstagedrc`를 생성하여 `apps/server` 및 `apps/web`에 대한 린팅 규칙 구성.

---

## 2단계: 기반 (블로킹 선행 조건)

**목적**: 특정 로직이 추가되기 전에 있어야 하는 핵심 훅 스크립트.

- [x] T004 빈 `.husky/pre-commit` 훅 파일 생성.
- [x] T005 빈 `.husky/pre-push` 훅 파일 생성.
- [x] T006 [P] 루트에서 `pnpm test` 명령이 작업 공간 전반에 걸쳐 vitest를 실행하는지 확인.

---

## 3단계: 사용자 스토리 1 (P1) - 프리커밋 코드 품질 검사

**목표**: 커밋 전에 Rust 포맷팅을 포함하여 스테이징된 파일의 린팅 오류를 자동으로 확인.
**독립적 테스트**: 문법 오류가 있는 파일 커밋 시도; 커밋 거부로 검증됨.

- [x] T007 [US1] 스테이징된 `*.{ts,tsx,js,jsx}` 파일에 대해 `eslint --fix` 및 `prettier --write`를 실행하도록 `.lintstagedrc` 구성.
- [x] T008 [US1] `npx lint-staged`를 실행하도록 `.husky/pre-commit` 업데이트.
- [x] T008-2 [US1] `apps/mobile/src-tauri`에서 `cargo fmt -- --check` 및 `cargo clippy` 호출을 포함하도록 `.husky/pre-commit` 업데이트.

---

## 4단계: 사용자 스토리 2 (P2) - 프리푸시 테스트 검증

**목표**: 푸시 전에 모든 단위 테스트가 통과(Rust 포함)하고 빌드가 성공하는지 확인.
**독립적 테스트**: 실패하는 테스트로 푸시; 푸시 거부로 검증됨.

- [x] T009 [US2] `pnpm test`를 실행하도록 `.husky/pre-push` 업데이트.
- [x] T009-2 [US2] `apps/mobile/src-tauri`에서 `cargo test`를 실행하도록 `.husky/pre-push` 업데이트.
- [x] T009-3 [US2] `pnpm --filter @z-project/web build`를 실행하도록 `.husky/pre-push` 업데이트.

---

## 5단계: 사용자 스토리 3 (P1) - CI/CD 품질 게이트

**목표**: 풀 리퀘스트에 대해 CI 파이프라인에서 품질 검사 시행.
**독립적 테스트**: 나쁜 코드로 PR 열기; CI 실패로 검증됨.

- [x] T010 [US3] `.github/workflows/ci.yml`이 모든 작업 공간에 대해 `lint` 및 `test`를 실행하는지 검토 및 확인 (이미 존재, 사용법 확인).

---

## 6단계: 마무리 및 공통 관심사

**목적**: 문서화 및 최종 검증.

- [ ] T011 [P] `README.md` 또는 `CONTRIBUTING.md`에 Git 훅에 대한 문서 추가.
- [x] T012 전체 워크플로우 검증: 파일 변경 -> 커밋(린트) -> 푸시(테스트) -> CI(린트+테스트).

---

## 의존성 및 실행 순서

- **1단계 & 2단계**는 Husky 설정을 위해 먼저 수행되어야 함.
- **3단계 (프리커밋)**는 1단계 & 2단계에 의존함.
- **4단계 (프리푸시)**는 1단계 & 2단계에 의존함.
- **5단계 (CI)**는 독립적이지만 개념적으로 여기에서 모니터링됨.

## 구현 전략

1.  Husky 및 lint-staged 설정 (1단계 & 2단계).
2.  프리커밋 검사 활성화 (3단계).
3.  프리푸시 검사 활성화 (4단계).
4.  CI 검증 (5단계).
