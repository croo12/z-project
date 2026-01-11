# 태스크: 서버 본격 운용 및 아티클 입력 기능

**입력**: `/specs/006-work-with-server/`의 설계 문서
**선행 조건**: plan.md, spec.md, research.md, data-model.md, contracts/

## 형식: `[ID] [P?] [Story] 설명`

- **[P]**: 병렬 진행 가능 (서로 다른 파일, 의존성 없음)
- **[Story]**: 이 태스크가 속한 사용자 스토리 (예: US1, US2, US3, US4)

---

## 1단계: 설정 (공유 인프라)

**목적**: 프로젝트 구조 확장 및 기본 설정

- [X] T001 `apps/server/data/` 디렉토리 생성 및 빈 `articles.json` 파일 초기화 (`{"articles": []}`)
- [X] T002 `apps/server/logs/` 디렉토리 생성 및 `.gitkeep` 파일 추가
- [X] T003 [P] `apps/server/package.json`에 PM2 의존성 추가 및 `start:prod` 스크립트 추가
- [X] T004 [P] `.gitignore`에 `apps/server/logs/*.log` 및 `apps/server/data/articles.json` 패턴 추가 (데이터 파일 제외)

---

## 2단계: 기반 (블로킹 선행 조건)

**목적**: 어떠한 사용자 스토리도 구현되기 전에 반드시 완료되어야 하는 핵심 인프라

**⚠️ 중요**: 이 단계가 완료될 때까지 사용자 스토리 작업을 시작할 수 없습니다.

- [X] T005 `apps/server/src/types/article.ts`에 ArticleMetadata, ArticleFeedback 타입 정의 생성
- [X] T006 `apps/server/src/lib/article-store.ts`에 ArticleStore 클래스 구현 (JSON 파일 기반 CRUD)
- [X] T007 `apps/server/src/lib/article-store.ts`에 피드백 관련 필드 (rating, positiveCount, negativeCount) 업데이트 로직 추가
- [X] T008 `apps/server/src/services/article.service.ts`에 ArticleService 클래스 스켈레톤 생성
- [X] T009 `apps/server/src/index.ts`에 환경변수 유효성 검사 로직 추가 (GEMINI_API_KEY 검증)

**체크포인트**: 기반 준비 완료 - 이제 사용자 스토리 구현을 시작할 수 있습니다.

---

## 3단계: 사용자 스토리 1 - 아티클 등록 (우선순위: P1) 🎯 MVP

**목표**: 아티클을 등록하고 벡터 DB에 저장하여 RAG 검색에 활용

**독립적 테스트**: `POST /articles`로 아티클 등록 후 `/query`로 검색하여 결과에 포함되는지 확인

### 사용자 스토리 1 구현

- [X] T010 [US1] `apps/server/src/services/article.service.ts`에 `createArticle()` 메서드 구현 (청크 분할, 벡터 저장, 메타데이터 저장)
- [X] T011 [US1] `apps/server/src/services/ingestion.service.ts`에 `ingestWithArticleId()` 메서드 추가 (articleId 메타데이터 포함)
- [X] T012 [US1] `apps/server/src/api/articles.ts` 파일 생성 및 `POST /` 라우터 구현
- [X] T013 [US1] `apps/server/src/api/articles.ts`에 요청 유효성 검사 추가 (title, url, content 필수)
- [X] T014 [US1] `apps/server/src/index.ts`에 `/articles` 라우터 등록
- [X] T015 [US1] 아티클 등록 API 수동 테스트 및 검증

**체크포인트**: 아티클 등록 후 `/query`로 검색하여 내용이 반환되는지 확인

---

## 4단계: 사용자 스토리 2 - 아티클 목록 조회 (우선순위: P2)

**목표**: 등록된 아티클 목록을 페이지네이션하여 조회

**독립적 테스트**: `GET /articles?page=1&limit=10`으로 목록 조회

### 사용자 스토리 2 구현

- [X] T016 [US2] `apps/server/src/services/article.service.ts`에 `listArticles()` 메서드 구현 (페이지네이션 지원)
- [X] T017 [US2] `apps/server/src/services/article.service.ts`에 `getArticleById()` 메서드 구현
- [X] T018 [US2] `apps/server/src/api/articles.ts`에 `GET /` 라우터 구현 (목록 조회)
- [X] T019 [US2] `apps/server/src/api/articles.ts`에 `GET /:id` 라우터 구현 (상세 조회)
- [X] T020 [US2] `apps/server/src/api/articles.ts`에 `DELETE /:id` 라우터 구현 (삭제)
- [X] T021 [US2] 아티클 목록/상세/삭제 API 수동 테스트 및 검증

**체크포인트**: 등록된 아티클 목록을 조회하고 상세 정보를 확인할 수 있음

---

## 5단계: 사용자 스토리 3 - 서버 상시 구동 설정 (우선순위: P1)

**목표**: PM2를 통해 서버를 백그라운드로 실행하고 자동 재시작 설정

**독립적 테스트**: `pm2 start ecosystem.config.js`로 서버 시작 후 `pm2 status`로 상태 확인

### 사용자 스토리 3 구현

- [X] T022 [US3] `apps/server/ecosystem.config.js` 파일 생성 (PM2 설정)
- [X] T023 [US3] `ecosystem.config.js`에 로그 파일 경로 설정 (`logs/` 디렉토리)
- [X] T024 [US3] `ecosystem.config.js`에 환경변수 로드 설정 (`.env` 파일 참조)
- [X] T025 [US3] `apps/server/package.json`에 `start:prod` 스크립트 추가 (`pm2 start ecosystem.config.js`)
- [X] T026 [US3] `specs/006-work-with-server/quickstart.md`에 PM2 명령어 가이드 업데이트
- [X] T027 [US3] PM2로 서버 시작/중지/재시작 수동 테스트 및 검증

**체크포인트**: `pnpm start:prod`로 서버 시작 후 `pm2 status`에서 running 상태 확인

---

## 6단계: 사용자 스토리 4 - 아티클 피드백 및 품질 개선 (우선순위: P2)

**목표**: 아티클에 대한 피드백을 수집하고 검색 가중치를 조정

**독립적 테스트**: `POST /articles/:id/feedback`으로 피드백 제출 후 아티클의 rating 변화 확인

### 사용자 스토리 4 구현

- [X] T028 [US4] `apps/server/src/services/article.service.ts`에 `submitFeedback()` 메서드 구현
- [X] T029 [US4] `apps/server/src/services/article.service.ts`에 `calculateRating()` 헬퍼 함수 구현 (점수 계산 공식 적용)
- [X] T030 [US4] `apps/server/src/services/article.service.ts`에 `updateVectorScores()` 메서드 구현 (벡터 청크의 retrieval_score_modifier 조정)
- [X] T031 [US4] `apps/server/src/lib/vector-store.ts`에 `updateScoresByArticleId()` 메서드 추가 (articleId로 일괄 업데이트)
- [X] T032 [US4] `apps/server/src/api/articles.ts`에 `POST /:id/feedback` 라우터 구현
- [X] T033 [US4] `apps/server/src/api/articles.ts`에 `GET /:id/feedback` 라우터 구현 (피드백 통계)
- [X] T034 [US4] 피드백 API 수동 테스트 및 검증 (rating 변화 확인)
- [X] T035 [US4] 피드백 후 `/query` 검색 결과 순위 변화 확인

**체크포인트**: 긍정적 피드백 후 해당 아티클의 rating 증가 및 검색 순위 상승 확인

---

## 7단계: 마무리 및 공통 관심사

**목적**: 여러 사용자 스토리에 영향을 미치는 개선 사항

- [X] T036 [P] `specs/006-work-with-server/quickstart.md` 문서 최종 검증 및 업데이트
- [X] T037 [P] `apps/server/README.md` 파일 생성 또는 업데이트 (API 문서화)
- [X] T038 코드 정리 및 리팩토링 (중복 코드 제거)
- [X] T039 모든 API 엔드포인트 수동 통합 테스트 수행
- [X] T040 `GEMINI.md` 파일에 본 기능 추가 기록

---

## 의존성 및 실행 순서

### 단계 의존성

- **설정 (1단계)**: 의존성 없음 - 즉시 시작 가능
- **기반 (2단계)**: 설정 완료에 의존 - 모든 사용자 스토리를 차단함
- **사용자 스토리 1 (3단계)**: 기반 단계 완료에 의존 - MVP
- **사용자 스토리 2 (4단계)**: 기반 단계 완료에 의존 - US1과 독립적
- **사용자 스토리 3 (5단계)**: 설정 단계 완료에 의존 - US1/US2와 독립적
- **사용자 스토리 4 (6단계)**: US1 완료에 의존 (아티클이 존재해야 피드백 가능)
- **마무리 (7단계)**: 모든 사용자 스토리 완료에 의존

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
     ┌─────────┼─────────┬─────────────┐
     ▼         ▼         ▼             │
┌────────┐ ┌────────┐ ┌────────┐       │
│ US1(P1)│ │ US2(P2)│ │ US3(P1)│       │
│ 아티클 │ │ 목록   │ │ PM2    │       │
│ 등록   │ │ 조회   │ │ 설정   │       │
└────┬───┘ └────────┘ └────────┘       │
     │                                 │
     ▼                                 │
┌────────┐                             │
│ US4(P2)│ ◄───────────────────────────┘
│ 피드백 │
└────────┘
     │
     ▼
┌────────┐
│ 마무리 │
└────────┘
```

### 병렬 기회

- **1단계**: T003, T004 병렬 실행 가능
- **2단계**: T005~T009 순차 실행 (의존성 있음)
- **3~6단계**: US1, US2, US3은 병렬 실행 가능 (US4는 US1 완료 후)
- **7단계**: T036, T037 병렬 실행 가능

---

## 병렬 예시: 사용자 스토리 1~3

```bash
# 기반 단계 완료 후 병렬 시작 가능:
개발자 A: 사용자 스토리 1 (T010~T015) - 아티클 등록
개발자 B: 사용자 스토리 2 (T016~T021) - 목록 조회
개발자 C: 사용자 스토리 3 (T022~T027) - PM2 설정

# 단일 개발자의 경우:
순서: 설정 → 기반 → US1 (MVP) → US3 → US2 → US4 → 마무리
```

---

## 구현 전략

### MVP 우선 (사용자 스토리 1 + 3)

1. **1단계**: 설정 완료
2. **2단계**: 기반 완료
3. **3단계**: 사용자 스토리 1 완료 (아티클 등록)
4. **5단계**: 사용자 스토리 3 완료 (PM2 설정)
5. **중지 및 검증**: 서버가 PM2로 실행되고 아티클을 등록/검색할 수 있는지 확인
6. 준비되면 배포/데모

### 점진적 전달

1. 설정 + 기반 완료 → 기반 준비됨
2. 사용자 스토리 1 + 3 추가 → 독립적 테스트 → **MVP 완료**
3. 사용자 스토리 2 추가 → 목록 조회 기능 추가
4. 사용자 스토리 4 추가 → 피드백 기반 품질 개선 기능 추가
5. 각 스토리는 이전 스토리를 깨뜨리지 않고 가치를 더함

---

## 참고

- [P] 태스크 = 다른 파일, 의존성 없음
- [Story] 라벨은 추적 가능성을 위해 태스크를 특정 사용자 스토리에 매핑함
- 각 사용자 스토리는 독립적으로 완료 가능하고 테스트 가능해야 함
- 각 태스크 또는 논리적 그룹 후 커밋
- 스토리 독립적 검증을 위해 각 체크포인트에서 중지
