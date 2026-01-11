# 구현 계획: 디버그 모드 UI 개선

**브랜치**: `008-debug-ui` | **날짜**: 2026-01-11 | **명세서**: [spec.md](./spec.md)
**입력**: `/specs/008-debug-ui/spec.md`의 기능 명세

## 요약

이 기능은 Tauri 모바일 앱의 디버그 빌드 환경에서 개발자가 서버 연결 상태와 AI 기능 동작 여부를 즉시 확인할 수 있는 화면 상단 오버레이 UI를 추가합니다.

## 기술적 맥락

**언어/버전**: TypeScript (React), Rust (Tauri)
**주요 의존성**: React, TailwindCSS, Tauri API
**대상 플랫폼**: Android (Debug Build)
**프로젝트 유형**: Mobile App Frontend & Core
**성능 목표**: 오버레이가 앱 성능에 영향을 주지 않아야 함 (가벼운 렌더링)
**제약 사항**: 프로덕션 빌드에서는 절대 노출되지 않아야 함 (`import.meta.env.DEV` 체크)

## 헌법(Constitution) 체크

| 원칙 | 상태 | 비고 |
|------|------|------|
| 1. 원칙 기반 AI 지원 개발 | ✅ 통과 | 본 계획문서가 개발 가이드 역할 |
| 2. 조합 가능한 프로젝트 명세 | ✅ 통과 | specs 디렉토리 내 모듈화된 명세 사용 |
| 3. 자동화 및 검증 가능한 워크플로우 | ✅ 통과 | 빌드 분리(Debug/Prod) 자동화 |
| 4. 구조화된 프로세스 지향 태스크 실행 | ✅ 통과 | speckit 워크플로우 준수 |
| 5. 한국어 우선 문서화 | ✅ 통과 | 본 문서 한국어 작성 |

## 프로젝트 구조

### 문서 (본 기능)

```text
specs/008-debug-ui/
├── plan.md              # 본 파일
├── spec.md              # 기능 명세
├── research.md          # 조사 (필요 시)
└── tasks.md             # 태스크 목록
```

### 소스 코드 (앱)

```text
apps/web/
├── src/
│   ├── components/
│   │   └── Debug/       # 신규: 디버그 관련 컴포넌트
│   │       ├── DebugOverlay.tsx
│   │       ├── ServerStatus.tsx
│   │       └── AITester.tsx
│   ├── App.tsx          # 수정: DebugOverlay 마운트
│   └── store/           # (선택) 상태 관리 필요 시
apps/mobile/src-tauri/
├── src/
│   ├── features/
│   │   └── debug/       # 신규: 디버그 전용 커맨드
│   │       ├── mod.rs
│   │       └── commands.rs
│   └── lib.rs           # 수정: 커맨드 등록
```

## 복잡도 추적

> 헌법 체크에 정당화가 필요한 위반 사항 없음
