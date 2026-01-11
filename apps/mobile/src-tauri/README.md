# Tauri Mobile Core (Rust)

Z-Project의 모바일 앱 백엔드(Rust)입니다.

## 주요 기능

- **추천 시스템**: Gemini AI 기반 아티클 추천
- **Sync 모듈**: AI Brain Server와의 데이터 동기화
- **SQLite 저장소**: 로컬 데이터 관리 (Todos, WorkLogs, Articles)

## 빌드 및 실행

```bash
pnpm tauri android dev
```

## 서버 동기화 설정

`.env` 파일에 `BRAIN_SERVER_URL`을 설정하여 외부 AI 서버와 연동할 수 있습니다.
동기화된 아티클은 서버의 RAG 지식 베이스로 활용됩니다.
