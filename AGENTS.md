# AI Agents Guide for z-project

## Project Vision
**"A curated platform for the continuous growth and self-improvement of a frontend developer."**

This project aims to build an application helps frontend developers stay updated with technology, analyze their work patterns, and manage tasks efficiently.

## Navigation
Depending on your task scope, refer to the specific guides below:

- **Frontend (React/Vite)**: [src/AGENTS.md](src/AGENTS.md)
- **Backend (Rust/Tauri)**: [src-tauri/AGENTS.md](src-tauri/AGENTS.md)

## Global Rules for Agents

### 1. Language Protocol
- **Implementation Plans**: Write in **Korean** (한국어).
- **PR Reviews / User Communication**: Write in **Korean** (한국어).
- **Code Comments**: English is preferred for code, but use Korean if it aids clarity for the user.

### 2. Documentation Maintenance
> [!IMPORTANT]
> **ALWAYS** create or update relevant documentation after completing a task.
- If you add a feature, update `AGENTS.md` or create a spec file in `specs/`.
- If you change a workflow, update `README.md` or `.github/workflows`.
- Never leave the documentation outdated relative to the code.

### 3. Workflow Discipline
- Review `task.md` and `implementation_plan.md` before starting work.
- Ensure `git status` is clean before starting a major task.

### 4. Technical Constraints
- Follow the specific tech stack rules in the sub-guides (Frontend/Backend).
- **Aesthetics**: Use modern, premium designs (Glassmorphism, etc.) for UI tasks.
