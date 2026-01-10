# 태스크: 한국어 문서 표준화

**입력**: `specs/003-korean-docs/`의 설계 문서
**선행 조건**: plan.md, spec.md

## 형식: `[ID] [P?] [Story?] 설명`

- **[P]**: 병렬 진행 가능
- **[Story]**: 해당 태스크가 속한 사용자 스토리

---

## 1단계: 템플릿 및 환경 설정 (최우선)

**목적**: 지금부터 생성되는 모든 문서가 즉시 한국어로 작성되도록 보장합니다.

- [x] T001 [US2] `.specify/templates/plan-template.md`를 한국어로 번역한다.
- [x] T002 [US2] `.specify/templates/spec-template.md`를 한국어로 번역한다.
- [x] T003 [US2] `.specify/templates/tasks-template.md`를 한국어로 번역한다.
- [x] T004 [US2] `.specify/templates/checklist-template.md`를 한국어로 번역한다.
- [x] T005 [US2] `AGENTS.md` 및 관련 프롬프트 파일에 한국어 출력을 강제하는 지침을 추가한다.

---

## 2단계: 기존 명세서 마이그레이션

**목적**: 현재 진행 중인 명세서들을 새로운 표준에 맞게 한국어로 전환합니다.

- [x] T006 [US1] [P] `specs/001-ai-brain-server/spec.md`를 한국어로 번역한다.
- [x] T007 [US1] [P] `specs/001-ai-brain-server/plan.md`를 한국어로 번역한다.
- [x] T008 [US1] [P] `specs/001-ai-brain-server/tasks.md`를 한국어로 번역한다.
- [x] T009 [US1] [P] `specs/002-quality-workflow/spec.md`를 한국어로 번역한다.
- [x] T010 [US1] [P] `specs/002-quality-workflow/plan.md`를 한국어로 번역한다.
- [x] T011 [US1] [P] `specs/002-quality-workflow/tasks.md`를 한국어로 번역한다.

---

## 3단계: 기타 및 검증

- [x] T012 [US1] [P] `specs/article-recommendation/*.md` 파일들이 활성 상태라면 번역한다.
- [x] T013 새로운 계획(Plan) 생성 시 한국어 템플릿이 로드되는지 검증한다.
