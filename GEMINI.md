# z-project Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-10

## Active Technologies

- TypeScript (Node.js v20+), Rust (latest stable) + LangChain.js, Express.js (001-ai-brain-server)

## Project Structure

```text
backend/
frontend/
tests/
```

## Commands

cargo test; cargo clippy

## Code Style

TypeScript (Node.js v20+), Rust (latest stable): Follow standard conventions

## Recent Changes

- 001-ai-brain-server: Added TypeScript (Node.js v20+), Rust (latest stable) + LangChain.js, Express.js
- 003-korean-docs: Standardized all documentation and templates to Korean. Enforced Korean-first policy.
- 004-shared-ai-package: Planned shared AI library (`packages/ai`) using LangChain.js to unify OpenAI and Gemini interactions.
- 005-migrate-server-to-gemini: Planned migration of `apps/server` to use Gemini via `packages/ai` and fix type errors.

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
