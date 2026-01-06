# Backend Agents Guide

## Project Vision
**"A curated platform for the continuous growth and self-improvement of a frontend developer."**
This app is designed to help the user (a frontend developer) stay updated with the latest technology, manage their work efficiently, and track their personal growth.

## Feature Specifications

### 1. Recommendation Engine (Rust + Gemini)
- **Logic**: Hybrid strategy (Rule-based + AI-based).
- **Data Source**: RSS Feeds (Rust, Android, React teams) + User Feedback History.
- **Gemini Integration**: Calls Gemini API to rank/select top 4 personalized articles.
- **Persistence**: SQLite (via `rusqlite`) stores articles and feedback.

### 2. Work Log Analytics
- **Commands**: `get_work_log_stats(period)` to return aggregated data.
- **Queries**: SQL `GROUP BY` operations for project distribution and daily trends.

### 3. Todo Management (Enhanced)
- **Data Model**: `TodoItem` struct extension.
  - `due_date`: Option<String> (ISO 8601)
  - `category`: Option<String>
- **Commands**: CRUD operations supporting new fields.

### 4. Android Signed Release
- **Keystore**: `release.jks` handling via GitHub Secrets (`base64` decoded in CI).
- **Signing**: Configured via environment variables injected during `tauri android build`.

## Project Scope
This directory (`src-tauri/`) contains the Backend application built with **Rust** and **Tauri v2**.

## Technology Stack
- **Framework**: Tauri v2
- **Language**: Rust (Edition 2021)
- **Database**: SQLite (`rusqlite`, `r2d2`)
- **HTTP Client**: `reqwest`

## Key Commands
- **Check Types**: `cargo check`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`
- **Run Dev**: `tauri dev` (from root) or `cargo run` (if pure rust logic)

## Project Structure
- Standard Rust binary crate structure.
- `src/lib.rs`: Library entry point (Tauri command registration).
- `src/main.rs`: Binary entry point.
- `src/features/`: Feature modules (e.g., `recommendation`, `worklog`).

## Critical Rules for Agents
1. **Context Awareness**: All backend code changes must be within `src-tauri/`.
2. **Error Handling**: Use `Result` and appropriate error types. Avoid `unwrap()` in production code.

### Pull Request (PR) Requirements
> [!IMPORTANT]
> Before submitting any changes or signaling completion, you **MUST** run and pass the following:

1. **Formatting**: `cargo fmt`
   - Ensure code is standard Rust formatted.
2. **Linting**: `cargo clippy`
   - Fix all warnings and errors.

**Do not submit a PR if these steps fail.**
