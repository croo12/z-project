# 구현 계획: 한국어 문서 표준화

**브랜치**: `003-korean-docs` | **날짜**: 2026-01-11 | **명세서**: [spec.md](./spec.md)
**입력**: `/specs/003-korean-docs/spec.md`의 기능 명세

## 요약

새로 비준된 헌법(Constitution)의 "한국어 우선 문서화(Korean-First Documentation)" 원칙에 따라, 프로젝트의 모든 기존 문서를 한국어로 번역하고, 향후 생성되는 산출물이 자연스럽게 한국어로 작성되도록 개발 환경(템플릿, 프롬프트)을 재구성합니다.

## 기술적 맥락

**언어/버전**: Markdown, 영어 -> 한국어 번역
**주요 의존성**: 없음 (텍스트 편집)
**대상 플랫폼**: 문서화 / 개발자 경험(DX)
**프로젝트 유형**: 문서 리팩토링

## 헌법(Constitution) 체크

*GATE: Phase 0 조사 전 통과 필수. Phase 1 설계 후 재확인.*

- **원칙 1 (규칙 기반 AI 개발)**: 헌법에 정의된 "한국어 우선" 규칙을 준수합니다.
- **원칙 5 (한국어 우선 문서화)**: 이 프로젝트는 해당 원칙을 구체적으로 실현하는 작업입니다.

**상태**: ✅ 통과

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/003-korean-docs/
├── plan.md              # 본 파일 (구현 계획서)
├── spec.md              # 기능 명세서
└── tasks.md             # 태스크 리스트
```

### 소스 코드 (영향 범위)

```text
.specify/templates/      # 번역 대상 템플릿 전체
specs/001-ai-brain-server/ # 번역 대상 기존 명세서
specs/002-quality-workflow/ # 번역 대상 기존 명세서
AGENTS.md                # 에이전트 지침 업데이트
```

## 단계(Phases)

### 1단계: 기반 마련 (템플릿 및 환경)

1.  **템플릿 번역**:
    -   `.specify/templates/plan-template.md`
    -   `.specify/templates/spec-template.md`
    -   `.specify/templates/tasks-template.md`
    -   `.specify/templates/checklist-template.md`
2.  **에이전트 컨텍스트 업데이트**:
    -   `AGENTS.md`를 수정하여 에이전트가 한국어를 출력하도록 명시적 지침 추가.
    -   (가능한 경우) `.specify` 프롬프트 검토 및 수정.

### 2단계: 마이그레이션 (활성 명세서)

1.  **001-ai-brain-server 번역**:
    -   `spec.md`, `plan.md`, `tasks.md` 번역.
2.  **002-quality-workflow 번역**:
    -   `spec.md`, `plan.md`, `tasks.md` 번역.

### 3단계: 검증

1.  **생성 테스트**: 모의 생성 명령을 실행하여 결과물이 한국어로 나오는지 확인.

## 복잡도 추적

| 위반 사항 | 필요한 이유 | 거부된 더 단순한 대안 |
|-----------|-------------|-----------------------|
| 없음 | | |
