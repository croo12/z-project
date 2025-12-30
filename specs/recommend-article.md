# 아티클 추천 기능 명세 (Article Recommendation Feature Specification)

## 1. 목표 (Goal)
개발자 사용자에게 React, Rust, Android, Tauri 등 관심 분야와 관련된 유용한 기술 아티클과 저널을 큐레이션하여 제공합니다. 이 기능을 통해 사용자가 앱 내에서 최신 정보를 얻고 영감을 받을 수 있도록 합니다.

## 2. 요구사항 (Requirements)

### 기능적 요구사항 (Functional Requirements)
1.  **동적 아티클 추천 (Dynamic Recommendation)**:
    -   정적 데이터가 아닌 외부 소스(RSS 등)에서 아티클을 가져와야 합니다.
    -   **제한 (Limit)**: **하루 총 7개** (Hybrid Strategy)
        -   **3개**: 최신순 & 미열람 우선 (Rule-based)
        -   **4개**: **Gemini AI**가 사용자 피드백 가중치를 기반으로 추천 (AI-based)
2.  **아티클 피드백 (User Feedback)**:
    -   사용자는 아티클을 읽은 후 "도움이 됨" / "도움이 안 됨"을 평가할 수 있습니다.
    -   평가 시 **이유**를 텍스트로 기록할 수 있어야 합니다.
3.  **개인화 추천 (Adaptive Recommendation)**:
    -   **Gemini Integration**: Rust 백엔드에서 Google Gemini API를 호출하여 추천 목록을 생성합니다.
    -   **Input**: 사용자 피드백 히스토리 + 미열람 아티클 목록.
    -   **Output**: 사용자 취향에 가장 잘 맞는 아티클 4개 선정.
4.  **카테고리**: React, Rust, Android, Tauri, TypeScript, General.
5.  **데이터 소스 (Data Sources Extensions)**:
    -   **Rust**: Official Blog, This Week in Rust (`https://this-week-in-rust.org/rss.xml`)
    -   **Android**: Android Developers, Android Weekly (`https://androidweekly.net/rss`), ProAndroidDev
    -   **React**: Official Blog, Overreacted (`https://overreacted.io/rss.xml`), React Native Blog
    -   **Tauri**: Official Blog
    -   **TypeScript**: Microsoft Blog (`https://devblogs.microsoft.com/typescript/feed/`)
    -   **General**: Hacker News (Optional)

### 비기능적 요구사항 (Non-Functional Requirements)
-   **데이터 영속성**: 피드백 및 캐시된 아티클 데이터는 로컬 DB(Sqlite) 또는 파일로 저장되어야 합니다.
-   **백그라운드 처리**: 데이터 갱신은 앱 실행 시 또는 백그라운드에서 비동기로 이루어져야 합니다.

## 3. 구현 계획 (Implementation Plan)

### 3.1 데이터 모델 (Rust)

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ArticleCategory {
    React,
    Rust,
    Android,
    Tauri,
    TypeScript,
    General,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub id: String, // UUID or Hash
    pub title: String,
    pub summary: String,
    pub url: String,
    pub category: ArticleCategory,
    pub published_at: String,
    pub feedback: Option<Feedback>, // 사용자의 피드백
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feedback {
    pub is_helpful: bool,
    pub reason: String,
    pub created_at: String, // Timestamp
}
```

### 3.2 백엔드 (Rust/Tauri)
*   **Storage**: `sqlite` 또는 `json` 파일 시스템을 사용하여 데이터를 저장합니다.
*   **Commands**:
    *   `fetch_articles(force: bool)`: RSS 피드 파싱 및 DB 업데이트 (24시간 체크).
    *   `get_recommended_articles()`: 피드백 알고리즘이 적용된 목록 반환.
    *   `submit_feedback(article_id: String, helpful: bool, reason: String)`: 피드백 저장 및 가중치 갱신.

### 3.3 추천 알고리즘 (V1)
*   간단한 가중치 시스템: 카테고리별 점수(Score)를 유지.
*   "도움이 됨" (+1점), "도움이 안 됨" (-1점).
*   상위 점수 카테고리의 아티클 비율을 높여서 반환.

### 3.4 프론트엔드 (React + TypeScript + CSS)
*   **피드백 UI**:
    *   아티클 하단 또는 모달에 👍 / 👎 버튼.
    *   클릭 시 이유 입력 폼 등장이 자연스럽게 처리.

## 4. 테스트 계획 (Testing Plan)

### 4.1 단위 테스트
*   `test_recommendation_algorithm`: 피드백 데이터에 따라 반환되는 아티클 구성이 변화하는지 검증.
*   `test_feedback_persistence`: 앱 재시작 후에도 피드백 기록이 남아있는지 확인.
*   `test_fetch_interval`: 마지막 갱신 시간이 24시간이 넘었을 때만 갱신 로직이 도는지 확인.

### 4.2 수동 검증
*   **피드백 반영 확인**: React 카테고리에 "도움이 됨"을 3번 연속 누른 후, `새로고침` 시 React 관련 글이 상단에 뜨거나 더 많이 나오는지 확인.
