# 구현 계획: 서버 본격 운용 및 아티클 입력 기능

**브랜치**: `006-work-with-server` | **날짜**: 2026-01-11 | **명세서**: [spec.md](./spec.md)
**입력**: `/specs/006-work-with-server/spec.md`의 기능 명세

## 요약

이 기능은 AI Brain Server를 본격적으로 운용하기 위한 설정을 완료하고, 아티클(기사, 블로그 포스트) 형태의 정보를 구조화하여 입력하는 API를 추가합니다. PM2를 통한 프로세스 관리, 아티클 전용 엔드포인트 구현, 메타데이터 저장소 추가가 핵심 작업입니다.

## 기술적 맥락

**언어/버전**: TypeScript (Node.js v20+)
**주요 의존성**: Express.js, LangChain.js, PM2, LanceDB
**저장소**: LanceDB (벡터), JSON 파일 (아티클 메타데이터)
**테스트**: Vitest (Unit & Integration)
**대상 플랫폼**: Windows / Node.js Server
**프로젝트 유형**: Backend Server (Monorepo)
**성능 목표**: 아티클 등록 응답 시간 < 2초
**제약 사항**: Windows 환경에서 PM2 구동, GEMINI_API_KEY 필수

## 헌법(Constitution) 체크

*GATE: Phase 0 조사 전 통과 필수. Phase 1 설계 후 재확인.*

| 원칙 | 상태 | 비고 |
|------|------|------|
| 1. 원칙 기반 AI 지원 개발 | ✅ 통과 | 본 계획문서가 개발 가이드 역할 |
| 2. 조합 가능한 프로젝트 명세 | ✅ 통과 | specs 디렉토리 내 모듈화된 명세 사용 |
| 3. 자동화 및 검증 가능한 워크플로우 | ✅ 통과 | PM2 ecosystem.config.js로 자동화 |
| 4. 구조화된 프로세스 지향 태스크 실행 | ✅ 통과 | speckit 워크플로우 준수 |
| 5. 한국어 우선 문서화 | ✅ 통과 | 본 문서 및 모든 산출물 한국어 작성 |

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/006-work-with-server/
├── plan.md              # 본 파일
├── spec.md              # 기능 명세
├── research.md          # Phase 0 출력
├── data-model.md        # Phase 1 출력
├── quickstart.md        # Phase 1 출력
├── contracts/           # Phase 1 출력
│   └── api.yml          # OpenAPI 스펙
└── tasks.md             # Phase 2 출력
```

### 소스 코드 (레포지토리 루트)

```text
apps/server/
├── src/
│   ├── api/
│   │   ├── articles.ts       # 신규: 아티클 전용 API
│   │   ├── knowledge.ts
│   │   ├── query.ts
│   │   ├── feedback.ts
│   │   └── interactions.ts
│   ├── services/
│   │   ├── article.service.ts  # 신규: 아티클 관리 서비스
│   │   └── ingestion.service.ts
│   ├── lib/
│   │   ├── vector-store.ts
│   │   ├── article-store.ts    # 신규: 아티클 메타데이터 저장소
│   │   └── logger.ts
│   └── index.ts
├── data/
│   └── articles.json          # 신규: 아티클 메타데이터 파일
├── logs/                       # 신규: 로그 디렉토리
├── ecosystem.config.js         # 신규: PM2 설정
└── package.json
```

**구조 결정**: 기존 모노레포 구조 유지, `apps/server` 내에 신규 파일 추가

## 복잡도 추적

> 헌법 체크에 정당화가 필요한 위반 사항 없음
