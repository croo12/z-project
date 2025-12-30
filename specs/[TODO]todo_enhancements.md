# [TODO] Todo 기능 고도화 (Todo Enhancements)

## 1. 개요 (Overview)
현재 Todo 기능은 단순한 "추가"와 "완료 토글"만 가능합니다. 실제 업무에서 유용하게 사용하기 위해 **수정, 삭제, 기한 설정, 카테고리 분류** 기능을 추가합니다.

## 2. 목표 (Goals)
-   사용자가 Todo 항목을 더 유연하게 관리할 수 있도록 합니다.
-   단순 리스트를 넘어선 작업 관리 도구로서의 가치를 제공합니다.

## 3. 상세 요구사항 (Detailed Requirements)

### 3.1 수정 (Edit)
-   **기능**: 이미 작성된 Todo의 텍스트를 수정할 수 있어야 합니다.
-   **UI**: Todo 아이템을 더블 클릭하거나, 수정 아이콘(✏️)을 눌러 인라인 편집 모드로 진입.

### 3.2 삭제 (Delete)
-   **기능**: 잘못 작성하거나 필요 없어진 Todo를 목록에서 영구 제거합니다.
-   **UI**: Todo 아이템 옆에 삭제 아이콘(🗑️) 배치. 삭제 시 확인 팝업(선택사항) 또는 즉시 삭제.

### 3.3 기한 설정 (Due Date)
-   **기능**: 각 Todo에 마감 기한을 설정할 수 있습니다.
-   **UI**:
    -   생성/수정 시 날짜 선택기(Date Picker) 제공.
    -   기한이 지난 항목은 붉은색 텍스트 등으로 강조 표시.
    -   "오늘 마감", "내일 마감" 등의 필터링 옵션 고려 (Future).

### 3.4 카테고리 (Category/Tags)
-   **기능**: Todo를 업무(Work), 개인(Personal), 학습(Study) 등으로 분류합니다.
-   **UI**:
    -   생성 시 드롭다운 또는 태그 입력.
    -   색상 코드로 카테고리 구분 (예: 업무=파랑, 개인=초록).

## 4. 데이터 모델 변경 (Rust/DB)

기존 `TodoItem` 구조체를 확장해야 합니다. (SQLite 테이블 스키마 변경 필요)

```rust
struct TodoItem {
    id: i32,
    text: String,
    completed: bool,
    // New Fields
    due_date: Option<String>, // ISO 8601 Date
    category: Option<String>,
}
```

## 5. 구현 계획
1.  **DB 스키마 변경**: `todos` 테이블에 `due_date`, `category` 컬럼 추가.
2.  **Rust Command 업데이트**:
    -   `update_todo(id, text, due_date, category)`
    -   `delete_todo(id)`
3.  **Frontend 업데이트**:
    -   Todo Item 컴포넌트 디자인 개선 (아이콘 추가, 레이아웃 변경).
    -   입력 폼에 옵션 필드 추가 (날짜, 카테고리).

## 6. 테스트 케이스
-   Todo 수정 후 새로고침 시 변경 내용 유지 확인.
-   Todo 삭제 시 목록에서 사라지고 DB에서도 제거 확인.
-   기한 설정 후 UI에 날짜 표시 확인.
