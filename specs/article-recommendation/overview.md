# 아티클 추천 기능 (Article Recommendation Overview)

## 개요
이 기능은 개발자 사용자에게 맞춤형 기술 뉴스 피드를 제공하는 것을 목표로 합니다. 단순한 RSS 리더가 아니라, 사용자의 명시적 관심사(Explicit Interests)와 행동 기반의 암시적 페르소나(Implicit Persona)를 결합하여 "딱 맞는" 콘텐츠를 추천합니다.

## 핵심 가치
1.  **관심사 기반**: Rust, React 등 내가 선택한 분야의 글만 본다.
2.  **노이즈 필터링**: 주식, 정치 등 불필요한 정보는 자동으로 걸러낸다.
3.  **지속적 학습**: 내가 글을 읽고 반응할수록 추천이 더 똑똑해진다.

## 문서 구조
이 폴더에는 다음과 같은 상세 명세가 포함되어 있습니다:

- **[requirements.md](./requirements.md)**: 상세 요구사항 정의서 (PRD). 기능적/비기능적 요구사항 및 사용자 스토리.
- **[roadmap.md](./roadmap.md)**: 단계별 구현 계획. 백엔드 DB 설정부터 프론트엔드 UI, AI 피드백 루프까지의 상세 개발 일정.

## 기술 스택
- **Backend**: Rust, Tauri, SQLite, Google Gemini API
- **Frontend**: React, TypeScript, TailwindCSS
- **AI**: Gemini 2.0 Flash (Context-aware Persona Update)
