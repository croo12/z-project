# 퀵스타트: 서버 본격 운용 및 아티클 입력 기능

**브랜치**: `006-work-with-server` | **날짜**: 2026-01-11

## 사전 요구사항

- Node.js v20 이상
- pnpm 패키지 매니저
- PM2 글로벌 설치 (`npm install -g pm2`)
- `GEMINI_API_KEY` 환경변수 설정

## 빠른 시작

### 1. 의존성 설치

```bash
cd /path/to/z-project
pnpm install
```

### 2. 환경변수 설정

`apps/server/.env` 파일 생성:

```env
GEMINI_API_KEY=your_gemini_api_key_here
PORT=3000
NODE_ENV=production
```

### 3. 서버 빌드

```bash
pnpm --filter @z-project/server build
```

### 4. 서버 시작 (개발 모드)

```bash
pnpm --filter @z-project/server dev
```

### 5. 서버 시작 (프로덕션 - PM2)

```bash
cd apps/server
pm2 start ecosystem.config.cjs
pm2 save
```

## API 사용 예시

### 아티클 등록

```bash
curl -X POST http://localhost:3000/articles \
  -H "Content-Type: application/json" \
  -d '{
    "title": "LangChain.js 시작하기",
    "url": "https://blog.example.com/langchain-intro",
    "content": "LangChain은 LLM 애플리케이션 개발을 위한 프레임워크입니다. 이 글에서는 LangChain.js의 기본 개념과 사용법을 알아봅니다.",
    "tags": ["langchain", "ai", "tutorial"]
  }'
```

### 아티클 목록 조회

```bash
curl http://localhost:3000/articles?page=1&limit=10
```

### 아티클 상세 조회

```bash
curl http://localhost:3000/articles/{article-id}
```

### 아티클 삭제

```bash
curl -X DELETE http://localhost:3000/articles/{article-id}
```

### 아티클 피드백 제출

```bash
# 긍정적 피드백
curl -X POST http://localhost:3000/articles/{article-id}/feedback \
  -H "Content-Type: application/json" \
  -d '{"type": "positive", "comment": "유용한 아티클입니다"}'

# 부정적 피드백
curl -X POST http://localhost:3000/articles/{article-id}/feedback \
  -H "Content-Type: application/json" \
  -d '{"type": "negative"}'
```

### 피드백 통계 조회

```bash
curl http://localhost:3000/articles/{article-id}/feedback
```

### 질의 (RAG)

```bash
curl -X POST http://localhost:3000/query \
  -H "Content-Type: application/json" \
  -d '{"query": "LangChain이란 무엇인가요?"}'
```

## PM2 명령어

### 기본 명령어

```bash
# 서버 시작
cd apps/server
pm2 start ecosystem.config.cjs

# 상태 확인
pm2 status

# 로그 확인 (실시간)
pm2 logs z-project-server

# 로그 확인 (최근 100줄)
pm2 logs z-project-server --lines 100

# 재시작
pm2 restart z-project-server

# 중지
pm2 stop z-project-server

# 삭제
pm2 delete z-project-server
```

### 프로세스 관리

```bash
# 현재 상태 저장 (재부팅 후 자동 시작)
pm2 save

# 저장된 프로세스 목록 복원
pm2 resurrect

# 모든 프로세스 중지
pm2 stop all

# 모든 프로세스 재시작
pm2 restart all
```

### 모니터링

```bash
# 실시간 모니터링 대시보드
pm2 monit

# 프로세스 상세 정보
pm2 show z-project-server
```

## 문제 해결

### GEMINI_API_KEY 오류

```
Error: Missing required environment variable: GEMINI_API_KEY
```

→ `.env` 파일에 유효한 API 키가 설정되어 있는지 확인하세요.

### 포트 충돌

```
Error: listen EADDRINUSE: address already in use :::3000
```

→ 이미 실행 중인 서버 프로세스가 있습니다. `pm2 list`로 확인 후 중지하거나 다른 포트를 사용하세요.

### 벡터 DB 초기화 실패

→ `apps/server/db/` 디렉토리의 권한을 확인하고, 필요 시 디렉토리를 삭제 후 재시작하세요.

### PM2 ecosystem 파일을 찾을 수 없음

```
Error: Script not found
```

→ `apps/server` 디렉토리에서 명령을 실행했는지 확인하세요. `ecosystem.config.cjs` 파일이 해당 디렉토리에 있어야 합니다.
