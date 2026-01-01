# 구현 로드맵 (Implementation Roadmap)

이 문서는 적응형 아티클 추천 시스템(Adaptive Article Recommendation System) 구현을 위한 단계별 개발 계획입니다.

## Phase 1: 기반 데이터 구조 및 백엔드 설정 (Backend Foundation)
데이터베이스 스키마를 확장하여 다중 태그와 중복 제거를 지원하고, 사용자의 명시적 관심사를 저장할 수 있는 구조를 만듭니다.

### Step 1.1: 데이터베이스 스키마 마이그레이션 (Multi-Tags)
- [ ] `src-tauri/src/db.rs` 업데이트.
- [ ] `articles` 테이블의 `category` 컬럼을 `tags` (TEXT/JSON)으로 변경하는 마이그레이션 스크립트 작성 (또는 테이블 재생성).
- [ ] Unique Key Constraint: `url` 컬럼에 UNIQUE 인덱스 추가 (중복 방지).

### Step 1.2: 로직 업데이트 (Fetch & Deduplicate)
- [ ] `src-tauri/src/features/recommendation/model.rs`: `Article` 구조체의 `category` -> `tags: Vec<ArticleCategory>` 변경.
- [ ] `fetch_feed`: 단일 카테고리 할당 대신 태그 리스트 생성 로직으로 변경.
- [ ] `fetch_articles`: `INSERT OR IGNORE` -> `INSERT ... ON CONFLICT(url) DO UPDATE SET tags = ...` 로직 구현 (태그 병합).

### Step 1.3: 사용자 설정 저장소 구현
- [ ] `RecommendationState` 확장: `UserPreferences` 저장/로드.
- [ ] Tauri Commands: `save_user_interests`, `get_user_interests`.

## Phase 2: 프론트엔드 설정 (Frontend Setup)
사용자가 자신의 관심사를 선택할 수 있는 UI를 구현합니다.

### Step 2.1: 카테고리 선택 UI 컴포넌트 개발
- [ ] `src/components/CategorySelector.tsx` 생성.
- [ ] 다중 선택 가능한 칩(Chip) 또는 체크박스 UI 구현.
- [ ] 선택된 카테고리 상태 관리 (React State).

### Step 2.2: 온보딩 모달/설정 화면 통합
- [ ] 앱 최초 실행 시 또는 설정 메뉴에서 `CategorySelector`를 띄울 수 있도록 라우팅/모달 연결.
- [ ] "저장" 버튼 클릭 시 백엔드의 `save_user_interests` 호출.

## Phase 3: 추천 알고리즘 고도화 (Algorithm Upgrade)
단순 RSS 페칭에서 선택된 카테고리와 AI 페르소나를 반영하는 하이브리드 로직으로 업그레이드합니다.

### Step 3.1: 명시적 필터링 로직 구현
- [ ] `src-tauri/src/features/recommendation/service.rs`의 `calculate_relevance_score` 함수 수정.
- [ ] 사용자가 선택한 카테고리에 해당하는 기사에 **높은 가중치(예: +50점)** 부여.
- [ ] 선택하지 않은 카테고리는 기본 점수 유지 또는 감점.

### Step 3.2: AI 페르소나 반영 로직 점검
- [ ] 현재 `recommend_with_gemini` 함수가 `UserPersona`를 잘 반영하고 있는지 확인.
- [ ] 프롬프트 튜닝: "사용자가 선택한 카테고리(Explicit)를 최우선으로 하되, 페르소나(Implicit)에 맞는 스타일의 글을 골라라"는 지시 추가.

### Step 3.3: 하이브리드 추천 파이프라인 통합
- [ ] `get_recommended_articles` 함수 흐름 재정의:
    1. DB에서 전체 기사 조회.
    2. 1차 필터: 사용자 선택 카테고리 위주로 기본 점수 산정.
    3. 상위 `N`개 후보 선정.
    4. 2차 필터(AI): 상위 후보 중 `UserPersona`와 가장 잘 맞는 4~5개 최종 선정.
    5. 최종 목록 반환.

## Phase 4: 피드백 루프 완성 (Feedback Loop)
사용자 반응을 수집하여 페르소나를 지속적으로 업데이트합니다.

### Step 4.1: 프론트엔드 피드백 UI 추가
- [ ] `src/components/ArticleList.tsx`의 각 기사 카드에 👍(좋아요) / 👎(관심없음) 버튼 추가.
- [ ] 버튼 클릭 시 `submit_feedback` 커맨드 호출 연결.
- [ ] (선택사항) 👎 클릭 시 "이유"를 묻는 팝오버/입력창 구현.

### Step 4.2: 백엔드 페르소나 업데이트 트리거 확인
- [ ] `submit_feedback` 함수 내에서 피드백 저장 후 `update_user_persona`가 비동기로 잘 호출되는지 검증.
- [ ] 잦은 업데이트 방지 로직(Debounce) 필요 여부 검토 (예: 피드백 5개 쌓일 때마다 업데이트).

## Phase 5: 테스트 및 검증 (Verification)

### Step 5.1: 단위/통합 테스트
- [ ] **백엔드**: 카테고리 선택 후 `get_recommended_articles` 호출 시 해당 카테고리 글이 상단에 오는지 테스트.
- [ ] **백엔드**: 피드백 생성 후 `UserPersona` 텍스트가 변경되는지 로그 확인.

### Step 5.2: UI/UX 테스트
- [ ] 온보딩 카테고리 선택 흐름의 사용성 테스트.
- [ ] 피드백 버튼 동작 확인 및 DB 반영 확인.
