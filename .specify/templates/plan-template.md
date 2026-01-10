# 구현 계획: [FEATURE]

**브랜치**: `[###-feature-name]` | **날짜**: [DATE] | **명세서**: [link]
**입력**: `/specs/[###-feature-name]/spec.md`의 기능 명세

**참고**: 이 템플릿은 `/speckit.plan` 명령에 의해 작성됩니다. 실행 워크플로우에 대해서는 `.specify/templates/commands/plan.md`를 참조하세요.

## 요약

[기능 명세서에서 추출: 주요 요구사항 + 조사를 통해 결정된 기술적 접근 방식]

## 기술적 맥락

<!--
  조치 필요: 이 섹션의 내용을 프로젝트의 기술적 세부 사항으로 교체하세요.
  여기에 제시된 구조는 반복 프로세스를 안내하기 위한 권장 사항입니다. 
-->

**언어/버전**: [예: Python 3.11, Swift 5.9, Rust 1.75 또는 확인 필요]  
**주요 의존성**: [예: FastAPI, UIKit, LLVM 또는 확인 필요]  
**저장소**: [해당되는 경우, 예: PostgreSQL, CoreData, files 또는 해당 없음]  
**테스트**: [예: pytest, XCTest, cargo test 또는 확인 필요]  
**대상 플랫폼**: [예: Linux server, iOS 15+, WASM 또는 확인 필요]
**프로젝트 유형**: [single/web/mobile - 소스 구조 결정]  
**성능 목표**: [도메인 특화, 예: 1000 req/s, 10k lines/sec, 60 fps 또는 확인 필요]  
**제약 사항**: [도메인 특화, 예: <200ms p95, <100MB memory, 오프라인 지원 또는 확인 필요]  
**규모/범위**: [도메인 특화, 예: 사용자 1만 명, 100만 줄의 코드, 화면 50개 또는 확인 필요]

## 헌법(Constitution) 체크

*GATE: Phase 0 조사 전 통과 필수. Phase 1 설계 후 재확인.*

[헌법 파일을 기반으로 결정된 게이트]

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/[###-feature]/
├── plan.md              # 본 파일 (/speckit.plan 명령 출력)
├── research.md          # Phase 0 출력 (/speckit.plan 명령)
├── data-model.md        # Phase 1 출력 (/speckit.plan 명령)
├── quickstart.md        # Phase 1 출력 (/speckit.plan 명령)
├── contracts/           # Phase 1 출력 (/speckit.plan 명령)
└── tasks.md             # Phase 2 출력 (/speckit.tasks 명령 - /speckit.plan으로 생성되지 않음)
```

### 소스 코드 (레포지토리 루트)
<!--
  조치 필요: 아래의 플레이스홀더 트리를 본 기능에 맞는 구체적인 레이아웃으로 교체하세요.
  사용하지 않는 옵션은 삭제하고 선택한 구조를 실제 경로(예: apps/admin, packages/something)로 확장하세요.
  전달되는 계획에는 옵션 레이블이 포함되어서는 안 됩니다.
-->

```text
# [사용하지 않을 경우 삭제] 옵션 1: 단일 프로젝트 (기본)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [사용하지 않을 경우 삭제] 옵션 2: 웹 애플리케이션 ("frontend" + "backend" 감지 시)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [사용하지 않을 경우 삭제] 옵션 3: 모바일 + API ("iOS/Android" 감지 시)
api/
└── [위의 backend와 동일]

ios/ 또는 android/
└── [플랫폼별 구조: 기능 모듈, UI 흐름, 플랫폼 테스트]
```

**구조 결정**: [선택한 구조를 문서화하고 위에 캡처된 실제 디렉토리를 참조하세요]

## 복잡도 추적

> **헌법 체크에 정당화가 필요한 위반 사항이 있는 경우에만 작성하세요**

| 위반 사항 | 필요한 이유 | 거부된 더 단순한 대안 |
|-----------|-------------|-----------------------|
| [예: 4번째 프로젝트] | [현재 필요성] | [3개 프로젝트가 불충분한 이유] |
| [예: 리포지토리 패턴] | [구체적인 문제] | [직접 DB 액세스가 불충분한 이유] |
