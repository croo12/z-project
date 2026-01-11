# 구현 계획: 앱-서버 아티클 동기화

**브랜치**: `007-app-server-sync` | **날짜**: 2026-01-11 | **명세서**: [spec.md](./spec.md)
**입력**: `/specs/007-app-server-sync/spec.md`의 기능 명세

## 요약

이 기능은 Tauri 안드로이드 앱에서 아티클을 읽을 때 해당 정보를 AI Brain Server로 전송하여 RAG 시스템에 지식을 축적합니다. 피드백도 동기화하여 서버의 검색 품질을 개선합니다.

## 기술적 맥락

**언어/버전**: Rust (latest stable)
**주요 의존성**: Tauri, reqwest, serde, rusqlite
**저장소**: SQLite (기존 DB 확장)
**테스트**: cargo test
**대상 플랫폼**: Android (Tauri Mobile)
**프로젝트 유형**: Mobile App (Monorepo - apps/mobile)
**성능 목표**: API 호출 타임아웃 5초 이내
**제약 사항**: 서버 연결 실패 시에도 앱 정상 동작 필수

## 헌법(Constitution) 체크

| 원칙 | 상태 | 비고 |
|------|------|------|
| 1. 원칙 기반 AI 지원 개발 | ✅ 통과 | 본 계획문서가 개발 가이드 역할 |
| 2. 조합 가능한 프로젝트 명세 | ✅ 통과 | specs 디렉토리 내 모듈화된 명세 사용 |
| 3. 자동화 및 검증 가능한 워크플로우 | ✅ 통과 | cargo test로 검증 |
| 4. 구조화된 프로세스 지향 태스크 실행 | ✅ 통과 | 단계별 구현 |
| 5. 한국어 우선 문서화 | ✅ 통과 | 본 문서 한국어 작성 |

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/007-app-server-sync/
├── plan.md              # 본 파일
├── spec.md              # 기능 명세
└── tasks.md             # 태스크 목록
```

### 소스 코드

```text
apps/mobile/src-tauri/
├── src/
│   ├── features/
│   │   ├── recommendation/
│   │   │   ├── commands.rs      # 수정: 아티클 동기화 호출 추가
│   │   │   ├── repository.rs    # 수정: 동기화 상태 추적 필드 추가
│   │   │   └── model.rs         # 수정: 동기화 관련 필드 추가 (선택적)
│   │   └── sync/                # 신규: 서버 동기화 모듈
│   │       ├── mod.rs
│   │       ├── client.rs        # 신규: Brain Server API 클라이언트
│   │       └── service.rs       # 신규: 동기화 서비스 로직
│   ├── lib.rs                   # 수정: sync 모듈 등록, 명령어 추가
│   └── error.rs                 # 수정: 동기화 에러 타입 추가 (선택적)
└── .env.example                  # 수정: BRAIN_SERVER_URL 추가
```

**구조 결정**: 기존 features 패턴 유지, 새 sync 모듈 추가

## 복잡도 추적

> 헌법 체크에 정당화가 필요한 위반 사항 없음
