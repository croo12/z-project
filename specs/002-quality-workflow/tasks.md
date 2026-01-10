# Tasks: Automated Code Quality Workflows

**Input**: Design documents from `specs/002-quality-workflow/`
**Prerequisites**: plan.md, spec.md

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel
- **[Story]**: User story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize configuration for git hooks and quality tools.

- [x] T001 Initialize Husky in root directory and add prepare script to package.json.
- [x] T002 Install lint-staged as a dev dependency in root.
- [x] T003 [P] Create `.lintstagedrc` in root to configure linting rules for `apps/server` and `apps/web`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core hook scripts that must be in place before specific logic is added.

- [x] T004 Create empty `.husky/pre-commit` hook file.
- [x] T005 Create empty `.husky/pre-push` hook file.
- [x] T006 [P] Verify `pnpm test` command runs vitest across workspaces from root.

---

## Phase 3: User Story 1 (P1) - Pre-commit Code Quality Checks

**Goal**: Automatically check staged files for linting errors before commit.
**Independent Test**: Attempt to commit a file with syntax errors; verified by commit rejection.

- [x] T007 [US1] Configure `.lintstagedrc` to run `eslint --fix` and `prettier --write` on staged `*.{ts,tsx,js,jsx}` files.
- [x] T008 [US1] Update `.husky/pre-commit` to execute `npx lint-staged`.

---

## Phase 4: User Story 2 (P2) - Pre-push Test Verification

**Goal**: Ensure all unit tests pass before pushing to remote.
**Independent Test**: Push with failing tests; verified by push rejection.

- [x] T009 [US2] Update `.husky/pre-push` to execute `pnpm test`.

---

## Phase 5: User Story 3 (P1) - CI/CD Quality Gate

**Goal**: Enforce quality checks in the CI pipeline for Pull Requests.
**Independent Test**: Open a PR with bad code; verified by CI failure.

- [x] T010 [US3] Review and confirm `.github/workflows/ci.yml` runs `lint` and `test` for all workspaces (already exists, verify usage).

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: documentation and final verification.

- [x] T011 [P] Add documentation about Git Hooks to `README.md` or `CONTRIBUTING.md`.
- [x] T012 Verify entire workflow: Change file -> Commit (Lint) -> Push (Test) -> CI (Lint+Test).

---

## Dependencies & Execution Order

- **Phase 1 & 2** must be done first to set up Husky.
- **Phase 3 (Pre-commit)** relies on Phase 1 & 2.
- **Phase 4 (Pre-push)** relies on Phase 1 & 2.
- **Phase 5 (CI)** is independent but conceptually monitored here.

## Implementation Strategy

1.  Set up Husky and lint-staged (Phase 1 & 2).
2.  Enable Pre-commit checks (Phase 3).
3.  Enable Pre-push checks (Phase 4).
4.  Verify CI (Phase 5).
