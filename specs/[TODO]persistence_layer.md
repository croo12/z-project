# [TODO] 데이터 영속성 레이어 도입 (Persistence Layer with SQLite)

## 1. 개요 (Overview)
현재 앱은 `Todos`와 `WorkLogs` 데이터를 메모리(`Mutex<Vec>`)에만 저장하고 있어, 앱 재시작 시 데이터가 유실됩니다. `Articles`의 경우 JSON 파일로 저장되지만, 데이터 관계와 확장성을 고려하여 **SQLite** 기반의 영속성 레이어를 도입해야 합니다.

## 2. 목표 (Goals)
1.  **데이터 보존**: 앱을 재시작해도 Todo, WorkLog, Article 데이터가 유지되어야 합니다.
2.  **안정성**: 파일 입출력 시 발생할 수 있는 데이터 손상(Race Condition 등)을 방지합니다.
3.  **확장성**: 향후 검색, 필터링, 정렬 기능을 효율적으로 구현할 수 있는 기반을 마련합니다.

## 3. 요구사항 (Requirements)

### 3.1 기술 스택
-   **Database**: SQLite
-   **Library**: `tauri-plugin-sql` (Tauri v2 공식 플러그인 권장) 또는 `rusqlite`.
    -   *Note*: Tauri v2 환경에 최적화된 방식을 선택합니다.

### 3.2 데이터베이스 스키마 설계
기존 데이터 구조를 관계형 테이블로 마이그레이션합니다.

#### Tables
1.  **todos**
    -   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
    -   `text` (TEXT NOT NULL)
    -   `completed` (BOOLEAN DEFAULT 0)
    -   `created_at` (DATETIME DEFAULT CURRENT_TIMESTAMP)

2.  **work_logs**
    -   `id` (INTEGER PRIMARY KEY AUTOINCREMENT)
    -   `project` (TEXT NOT NULL)
    -   `hours` (REAL NOT NULL)
    -   `date` (TEXT NOT NULL) -- YYYY-MM-DD format
    -   `created_at` (DATETIME DEFAULT CURRENT_TIMESTAMP)

3.  **articles**
    -   `id` (TEXT PRIMARY KEY) -- URL Hash or GUID
    -   `title` (TEXT)
    -   `summary` (TEXT)
    -   `url` (TEXT)
    -   `category` (TEXT)
    -   `published_at` (TEXT)
    -   `feedback_helpful` (BOOLEAN NULL)
    -   `feedback_reason` (TEXT NULL)
    -   `feedback_at` (TEXT NULL)

### 3.3 백엔드 변경 사항 (Rust)
-   `AppState`에서 `Mutex<Vec<...>>` 제거.
-   대신 `db: tauri_plugin_sql::Builder` 또는 Connection Pool을 관리.
-   각 Command (`get_todos`, `add_todo` 등) 내부 로직을 SQL 쿼리 실행으로 변경.

## 4. 구현 단계 (Implementation Steps)
1.  **의존성 추가**: `Cargo.toml`에 `tauri-plugin-sql` 및 `sqlite` 드라이버 추가.
2.  **플러그인 설정**: `main.rs` (or `lib.rs`)의 `tauri::Builder`에 플러그인 등록.
3.  **마이그레이션 스크립트**: 앱 최초 실행 시 테이블 생성 SQL 실행.
4.  **Command 리팩토링**:
    -   `todos` 관련 함수 DB 연결.
    -   `work_logs` 관련 함수 DB 연결.
    -   `articles` 관련 함수 DB 연결.
5.  **테스트**: 데이터 추가 후 앱 재시작하여 보존 여부 확인.

## 5. 예상 효과 (Expected Benefits)
-   사용자가 작성한 투두리스트와 업무 로그가 안전하게 저장됩니다.
-   수천 건의 로그가 쌓여도 효율적인 조회가 가능해집니다.
