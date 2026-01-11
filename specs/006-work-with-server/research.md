# 조사 문서: 서버 본격 운용 및 아티클 입력 기능

**브랜치**: `006-work-with-server` | **날짜**: 2026-01-11

## 조사 항목

### 1. PM2 Windows 환경 설정

**결정**: PM2를 사용하여 Node.js 서버를 백그라운드 프로세스로 관리

**근거**:
- PM2는 Node.js 프로세스 관리자로 Windows, macOS, Linux 모두 지원
- 자동 재시작, 로그 관리, 클러스터 모드 등 프로덕션급 기능 제공
- `ecosystem.config.js` 파일로 설정을 코드화하여 버전 관리 가능

**고려된 대안**:
- Windows Task Scheduler: Node.js 특화 기능 부족
- Docker: 로컬 개발 환경에서는 오버엔지니어링
- nodemon: 개발용이며 프로덕션 용도로 부적합

**적용 방법**:
```bash
npm install -g pm2
pm2 start ecosystem.config.js
pm2 save
pm2-startup  # Windows에서는 별도 설정 필요
```

### 2. 아티클 메타데이터 저장소

**결정**: JSON 파일 기반 단순 저장소 사용 (`data/articles.json`)

**근거**:
- 현재 단계에서는 복잡한 DB 설정 불필요
- JSON 파일은 버전 관리 가능하고 디버깅 용이
- 향후 SQLite 또는 다른 DB로 마이그레이션 용이한 추상화 레이어 적용

**고려된 대안**:
- SQLite: 작은 규모에서는 오버헤드
- LanceDB 메타데이터: 벡터 DB는 메타데이터 쿼리에 최적화되지 않음
- MongoDB: 외부 서비스 의존성 증가

**데이터 구조**:
```json
{
  "articles": [
    {
      "id": "uuid",
      "title": "아티클 제목",
      "url": "https://example.com/article",
      "tags": ["tag1", "tag2"],
      "createdAt": "2026-01-11T12:00:00Z",
      "chunkCount": 5
    }
  ]
}
```

### 3. 아티클 API 설계

**결정**: RESTful API 패턴 적용

**근거**:
- 기존 `/knowledge` API와 일관성 유지
- 직관적이고 표준화된 인터페이스
- 클라이언트 확장 시 호환성 확보

**엔드포인트 설계**:
| 메서드 | 경로 | 설명 |
|--------|------|------|
| POST | /articles | 새 아티클 등록 |
| GET | /articles | 아티클 목록 조회 |
| GET | /articles/:id | 특정 아티클 상세 조회 |
| DELETE | /articles/:id | 아티클 삭제 |

### 4. 로깅 전략

**결정**: Pino 로거 유지, 파일 출력 추가

**근거**:
- 기존 Pino 로거 활용하여 일관성 유지
- `pino-pretty`는 개발용, 프로덕션에서는 JSON 포맷 파일 출력
- PM2 자체 로그 관리 기능과 병행 사용

**적용 방법**:
- `logs/` 디렉토리에 일별 로그 파일 저장
- PM2의 `--log-date-format` 옵션 활용

## 기술적 결정 요약

| 항목 | 결정 | 이유 |
|------|------|------|
| 프로세스 관리 | PM2 | Windows 호환, 자동 재시작, 로그 관리 |
| 메타데이터 저장소 | JSON 파일 | 단순성, 버전 관리 용이 |
| API 패턴 | REST | 기존 API와 일관성 |
| 로깅 | Pino + 파일 | 기존 설정 활용, 프로덕션 대응 |
