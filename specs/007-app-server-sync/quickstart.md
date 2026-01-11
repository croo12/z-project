# 아티클 동기화 퀵스타트

이 문서는 Tauri 앱에서 AI Brain Server로 아티클 및 피드백을 동기화하는 기능을 설정하고 테스트하는 방법을 설명합니다.

## 1. 서버 준비

AI Brain Server가 실행 중이어야 합니다 (`006-work-with-server` 참고).

```bash
cd apps/server
pnpm dev
```

## 2. 앱 설정

`apps/mobile/src-tauri/.env` 파일에 서버 URL을 설정합니다.

```env
BRAIN_SERVER_URL=http://localhost:3000
```

## 3. 기능 작동 방식

### 아티클 동기화
1. 앱의 'Recommendations' 목록에서 아티클을 선택합니다.
2. 'Read Article' 링크를 클릭합니다.
3. 클릭 시 `sync_article_to_server` 명령어가 호출되어 서버에 아티클 정보가 전송됩니다.
4. 중복 전송 방지를 위해 SQLite DB에 전송 상태가 저장됩니다.

### 피드백 동기화
1. 이미 읽은(전송된) 아티클에 대해 'Rate this article'을 클릭합니다.
2. Helpful/Not Helpful 및 이유를 입력하고 제출합니다.
3. 로컬 DB 업데이트와 동시에 서버의 피드백 API가 호출됩니다.
4. 서버의 RAG 검색 가중치가 실시간으로 조정됩니다.

## 4. 수동 테스트 방법

### 서버 헬스체크 테스트
Tauri 명령어 `check_server_health`를 호출하여 서버 연결 상태를 확인합니다.

### 데이터 확인
서버의 로그 또는 API를 통해 데이터 전송을 확인합니다.

```bash
# 서버에 저장된 아티클 목록 확인
curl http://localhost:3000/articles
```

### RAG 검색 확인
동기화된 아티클 내용을 기반으로 질의를 수행합니다.

```bash
curl -X POST http://localhost:3000/query \
  -H "Content-Type: application/json" \
  -d '{"query": "앱에서 방금 읽은 내용에 대해 알려줘"}'
```
