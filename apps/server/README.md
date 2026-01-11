# AI Brain Server

AI 기반 지식 관리 서버 - RAG(Retrieval-Augmented Generation) 시스템을 통한 아티클 관리 및 검색

## 개요

이 서버는 아티클(기사, 블로그 포스트 등)을 수집하고, 벡터 데이터베이스에 저장하여 자연어 질의에 대한 컨텍스트 기반 응답을 제공합니다.

## 기능

- **아티클 관리**: 등록, 조회, 삭제
- **피드백 시스템**: 아티클 평가 및 검색 가중치 자동 조정
- **RAG 검색**: 자연어 질의에 대한 관련 아티클 기반 응답
- **PM2 통합**: 프로덕션 환경에서의 안정적인 서버 운영

## API 엔드포인트

### 아티클 (`/articles`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/articles` | 새 아티클 등록 |
| GET | `/articles` | 아티클 목록 조회 (페이지네이션) |
| GET | `/articles/:id` | 아티클 상세 조회 |
| DELETE | `/articles/:id` | 아티클 삭제 |
| POST | `/articles/:id/feedback` | 피드백 제출 |
| GET | `/articles/:id/feedback` | 피드백 통계 조회 |

### 질의 (`/query`)

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/query` | RAG 기반 질의 처리 |

### 기타

| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | `/knowledge` | 일반 지식 등록 |
| POST | `/feedback` | 일반 피드백 제출 |
| POST | `/interactions` | 상호작용 로깅 |

## 기술 스택

- **Runtime**: Node.js v20+
- **Language**: TypeScript
- **Framework**: Express.js
- **AI**: LangChain.js, Google Gemini API
- **Vector DB**: LanceDB
- **Process Manager**: PM2

## 설치 및 실행

### 사전 요구사항

```bash
# Node.js v20+
node --version

# PM2 글로벌 설치
npm install -g pm2
```

### 환경변수 설정

`.env` 파일 생성:

```env
GEMINI_API_KEY=your_api_key_here
PORT=3000
```

### 개발 모드

```bash
# 루트 디렉토리에서
pnpm --filter @z-project/server dev
```

### 프로덕션 모드

```bash
# 빌드
pnpm --filter @z-project/server build

# PM2로 시작
cd apps/server
pm2 start ecosystem.config.cjs
```

## 데이터 구조

### 아티클 등록 요청

```json
{
  "title": "아티클 제목",
  "url": "https://example.com/article",
  "content": "아티클 본문 내용...",
  "tags": ["tag1", "tag2"]
}
```

### 피드백 요청

```json
{
  "type": "positive",
  "comment": "유용한 아티클입니다"
}
```

## 파일 구조

```
apps/server/
├── src/
│   ├── api/           # API 라우터
│   │   ├── articles.ts
│   │   ├── query.ts
│   │   └── ...
│   ├── services/      # 비즈니스 로직
│   │   ├── article.service.ts
│   │   └── ingestion.service.ts
│   ├── lib/           # 유틸리티
│   │   ├── article-store.ts
│   │   ├── vector-store.ts
│   │   └── logger.ts
│   ├── types/         # 타입 정의
│   │   └── article.ts
│   └── index.ts       # 엔트리포인트
├── data/              # 아티클 메타데이터
├── logs/              # 로그 파일
├── db/                # 벡터 데이터베이스
└── ecosystem.config.cjs  # PM2 설정
```

## 라이선스

ISC
