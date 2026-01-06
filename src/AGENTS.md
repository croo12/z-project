# Frontend Agents Guide

## Project Vision
**"A curated platform for the continuous growth and self-improvement of a frontend developer."**
This app is designed to help the user (a frontend developer) stay updated with the latest technology, manage their work efficiently, and track their personal growth.

## Feature Specifications

### 1. Article Recommendation
- **Goal**: Curate high-quality tech articles (React, Rust, Android, Tauri) to inspire and educate.
- **Mechanism**:
  - Displays 7 articles daily: 3 based on recency/unread status, 4 curated by **Gemini AI** based on user feedback.
  - **Feedback UI**: Helpful/Not Helpful buttons with reason input.

### 2. Work Log Analytics
- **Goal**: Visualize work patterns to improve efficiency.
- **UI**:
  - **Dashboard**: "Today: X hours", "This Week: Y hours".
  - **Charts**: Project distribution (Pie/Donut), Daily trend (Bar) using `recharts` or similar.

### 3. Todo Management (Enhanced)
- **Goal**: Flexible task management beyond simple lists.
- **Features**:
  - **Edit/Delete**: Inline editing and removal.
  - **Due Dates**: Date picker integration.
  - **Categories**: Tagging system (Work, Personal, Study).

### 4. Release & Delivery
- **Goal**: Automated signed releases for Android.
- **CI/CD**: GitHub Actions pipeline triggering `npm run build` and checking linting rules.

## Project Scope
This directory (`src/`) contains the Frontend application built with **React 19**, **Vite**, and **Tailwind CSS**.

## Technology Stack
- **Framework**: React 19
- **Build Tool**: Vite
- **Styling**: Tailwind CSS, Radix UI (Primitives)
- **Language**: TypeScript

## Key Commands
- **Start Dev Server**: `npm run dev`
- **Build**: `npm run build`
- **Lint**: `npm run lint`

## Project Structure
- Follows a modified Feature-Sliced Design (FSD).
- Top-level directories: `components`, `hooks`, `utils`, `types`, `recoil`.
- Recommendations:
    - Generic UI components -> `shared/ui`
    - Business logic hooks/utils/types -> Feature/Entity slices or `shared/lib`
    - Recoil setup -> `app/providers`

## Critical Rules for Agents
1. **Context Awareness**: All frontend code changes must be within `src/`. Do not touch `src-tauri/` unless specifically instructed for backend integration.
2. **Component usage**: Prefer using existing UI components from `src/components` or `src/shared/ui` over creating duplicates.

### Pull Request (PR) Requirements
> [!IMPORTANT]
> Before submitting any changes or signaling completion, you **MUST** run and pass the following:

1. **Linting**: `npm run lint`
   - Fix all reported errors.
2. **Build**: `npm run build`
   - Ensure the project builds without errors.

**Do not submit a PR if these steps fail.**
