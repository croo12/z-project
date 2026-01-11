# z-project Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-11

## Active Technologies

- TypeScript (Node.js v20+), Rust (latest stable) + LangChain.js, Express.js (001-ai-brain-server)
- PM2, LanceDB, Google Gemini API (006-work-with-server)

## Project Structure

```text
apps/
  server/     # AI Brain Server (Node.js/TypeScript)
  web/        # Web Frontend (React)
  mobile/     # Mobile App (Tauri/Rust)
packages/
  ai/         # Shared AI utilities
specs/        # Feature specifications
```

## Commands

```bash
# Rust tests
cargo test; cargo clippy

# Server development
pnpm --filter @z-project/server dev

# Server production (PM2)
cd apps/server && pm2 start ecosystem.config.cjs

# Build all
pnpm build
```

## Code Style

TypeScript (Node.js v20+), Rust (latest stable): Follow standard conventions

## Recent Changes

- 001-ai-brain-server: Added TypeScript (Node.js v20+), Rust (latest stable) + LangChain.js, Express.js
- 003-korean-docs: Standardized all documentation and templates to Korean. Enforced Korean-first policy.
- 006-work-with-server: Added article management API, feedback system, PM2 production setup
- 007-app-server-sync: Implemented article and feedback synchronization between Tauri app and AI Brain Server

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->

