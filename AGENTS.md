# AI Agents Guide for z-project

## Project Vision
**"A curated platform for the continuous growth and self-improvement of a frontend developer."**

This project aims to build an application helps frontend developers stay updated with technology, analyze their work patterns, and manage tasks efficiently.

## Navigation
Depending on your task scope, refer to the specific guides below:

- **Frontend (React/Vite)**: [apps/web/AGENTS.md](apps/web/AGENTS.md)
- **Backend (Rust/Tauri)**: [apps/mobile/src-tauri/AGENTS.md](apps/mobile/src-tauri/AGENTS.md)

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

### 5. MCP Usage Guidelines
- **Context7**: Use this when you need up-to-date documentation, library usage examples, or when your internal knowledge might be outdated.
- **Sequential Thinking**: Use this for complex problem solving, architectural decisions, breaking down large tasks, or when you feel "stuck" or need to verify your logic.
- **Spec-Kit**: MANTATORY for all new features, significant refactoring, or multi-file changes. Use it to generate specs, plans, and task lists before writing code.

