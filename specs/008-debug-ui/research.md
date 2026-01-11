# 조사 문서: 디버그 모드 UI 개선

**브랜치**: `008-debug-ui` | **날짜**: 2026-01-11

## 조사 항목

### 1. Tauri 디버그 모드 감지 방법

**결정**: `import.meta.env.DEV` 사용

**근거**:
- Vite 기반 프로젝트이므로 `import.meta.env.DEV`가 가장 표준적이고 확실한 방법입니다.
- Tauri의 `#[cfg(debug_assertions)]`는 백엔드(Rust) 로직 분기에 사용하고, 프론트엔드는 Vite 환경변수를 따르는 것이 자연스럽습니다.

**적용 방법**:
```typescript
{import.meta.env.DEV && <DebugOverlay />}
```

### 2. 오버레이 UI 구현 (Draggable vs Fixed)

**결정**: 초기에는 **Fixed (Top/Collapsed)** 방식 사용

**근거**:
- Draggable 구현은 복잡도를 높임 (터치 이벤트 처리 등).
- 화면 상단에 얇은 바(Bar) 형태로 접혀 있다가, 클릭 시 확장되는 Accordion 스타일이 모바일에서 가장 효율적입니다.
- `position: fixed; top: 0; left: 0; z-index: 9999;` 스타일 적용.

### 3. Tauri 커맨드 구조

**결정**: 별도 `debug` 모듈 분리

**근거**:
- 디버깅용 코드가 프로덕션 코드와 섞이는 것을 방지하기 위해 `src/features/debug` 모듈로 격리합니다.
- `lib.rs`에서도 `#[cfg(debug_assertions)]`를 사용하여 릴리즈 빌드 시 해당 모듈이 아예 컴파일되지 않도록 합니다.

**구조**:
```rust
#[cfg(debug_assertions)]
pub mod debug;
```

## 기술적 결정 요약

| 항목 | 결정 | 이유 |
|------|------|------|
| 모드 감지 | `import.meta.env.DEV` | Vite 표준 준수 |
| UI 스타일 | Fixed Top Collapsible | 모바일 화면 공간 절약 |
| 백엔드 구조 | `features/debug` 모듈 분리 | 프로덕션 빌드 영향 최소화 |
