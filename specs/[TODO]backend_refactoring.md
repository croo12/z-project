# [TODO] 백엔드 리팩토링 및 모듈화 (Backend Refactoring)

## 1. 개요 (Overview)
현재 `src-tauri/src/lib.rs` 파일 하나에 모든 로직(Todo, WorkLog, News, AI Recommendation, State Management)이 포함되어 있습니다. 코드베이스가 커짐에 따라 유지보수와 확장성이 떨어지므로, **기능별 모듈 분리**가 필요합니다.

## 2. 목표 (Goals)
-   **관심사의 분리**: 각 기능(도메인)별로 코드를 독립적인 파일로 관리합니다.
-   **가독성 향상**: 파일 크기를 줄이고 코드 탐색을 용이하게 합니다.
-   **협업 용이성**: 여러 개발자가 서로 다른 기능을 동시에 수정할 때 충돌을 최소화합니다.

## 3. 리팩토링 구조 (Proposed Structure)

`src-tauri/src/` 내부 구조를 다음과 같이 변경합니다.

```
src-tauri/src/
├── lib.rs          # Entry point, Command 등록, App Setup
├── db.rs           # DB Connection 및 공통 설정 (Persistence 도입 시)
└── modules/        # 도메인별 모듈 디렉토리
    ├── mod.rs      # 모듈 공개 설정
    ├── common.rs   # 공통 타입 (Error 등)
    ├── todo.rs     # Todo 관련 Struct, Impl, Commands
    ├── worklog.rs  # WorkLog 관련 Struct, Impl, Commands
    └── news.rs     # Article, RSS Fetching, Gemini Recommendation
```

## 4. 상세 작업 내용

### 4.1 `todo.rs`
-   `TodoItem` struct 이동.
-   `get_todos`, `add_todo`, `toggle_todo` (및 향후 `update`, `delete`) 함수 이동.

### 4.2 `worklog.rs`
-   `WorkLog` struct 이동.
-   `get_work_logs`, `add_work_log` 함수 이동.

### 4.3 `news.rs`
-   `NewsItem`, `Article`, `ArticleCategory`, `Feedback` struct 이동.
-   RSS Fetching 로직 (`fetch_feed`, `fetch_articles`) 이동.
-   Gemini Recommendation 로직 (`recommend_with_gemini`, `get_recommended_articles`) 이동.

### 4.4 `lib.rs` 정리
-   각 모듈을 `mod modules;`로 불러옵니다.
-   `invoke_handler` 매크로에서 `modules::todo::add_todo`와 같이 경로를 지정하여 등록합니다.

## 5. 기대 효과
-   새로운 기능(예: Todo Enhancements) 개발 시 `todo.rs`만 수정하면 되므로 사이드 이펙트가 줄어듭니다.
-   테스트 코드도 각 모듈 파일 하단(`mod tests`)에 위치시켜 관리가 수월해집니다.
