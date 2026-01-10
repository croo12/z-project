# Tasks: AI Brain Server

**Input**: Design documents from `C:\Users\jimmy\project\z-project\specs\001-ai-brain-server`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel
- **[Story]**: User story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic tooling setup for `apps/server`.

- [ ] T001 Initialize a new Node.js project in `apps/server/package.json`.
- [ ] T002 [P] Add and configure ESLint and Prettier for code quality in `apps/server/`.
- [ ] T003 [P] Add and configure Vitest for testing in `apps/server/vitest.config.ts`.
- [ ] T004 [P] Initialize a new Rust library project in `apps/server/rust_components/` for the N-API module.
- [ ] T005 [P] Add `napi-rs` as a dependency to `apps/server/rust_components/Cargo.toml`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before user stories can be implemented.

- [ ] T006 Setup a basic Express.js server in `apps/server/src/index.ts`.
- [ ] T007 Define core data types (Interaction, Feedback) based on the data model in `apps/server/src/types/index.ts`.
- [ ] T008 [P] Create a wrapper service for the FAISS vector store in `apps/server/src/lib/vector-store.ts`, responsible for loading and saving the index.
- [ ] T009 [P] Create the basic structure for the LangGraph graph in `apps/server/src/core/graph.ts`.
- [ ] T010 Define the N-API interface in `apps/server/rust_components/src/lib.rs` and `apps/server/src/lib/rust-addon.ts` to expose Rust functions to Node.js.

---

## Phase 3: User Story 2 (P1) - Real-time Knowledge Ingestion

**Goal**: Implement the ability to add new information to the knowledge base in real-time.
**Independent Test**: Send a document to the `/knowledge` endpoint, then verify a subsequent `/query` call can retrieve that information.

- [ ] T011 [US2] Create the API endpoint for knowledge ingestion at `POST /knowledge` in `apps/server/src/api/knowledge.ts`.
- [ ] T012 [US2] Implement the `IngestionService` in `apps/server/src/services/ingestion.service.ts` to handle text splitting, embedding generation, and adding documents to the FAISS store via the wrapper.

---

## Phase 4: User Story 1 (P1) - Contextual Interaction & Feedback

**Goal**: Implement the ability to log user-AI interactions and receive feedback on them.
**Independent Test**: Call the `/interactions` endpoint, receive an ID, then call the `/feedback` endpoint with that ID and verify the data is stored.

- [ ] T013 [P] [US1] Create the API endpoint for logging interactions at `POST /interactions` in `apps/server/src/api/interactions.ts`.
- [ ] T014 [P] [US1] Create the API endpoint for submitting feedback at `POST /feedback` in `apps/server/src/api/feedback.ts`.
- [ ] T015 [US1] Implement the `FeedbackService` in `apps/server/src/services/feedback.service.ts`. Initially, this service will just store the interaction and feedback data. (Score adjustment logic will be in US3).

---

## Phase 5: User Story 3 (P2) - Enhanced AI Responses

**Goal**: Use the stored knowledge and feedback to provide augmented responses to user queries.
**Independent Test**: With known, unique information in the vector store, call `/query` with a related question and verify the unique information is present in the response.

- [ ] T016 [US3] Implement the core RAG logic within the LangGraph graph in `apps/server/src/core/graph.ts`, including retrieval from FAISS.
- [ ] T017 [US3] In the `FeedbackService`, add the logic to adjust the `retrieval_score_modifier` in the metadata of `KnowledgeChunk`s based on user feedback. The RAG logic in `T016` should use this score.
- [ ] T018 [US3] Create the API endpoint for querying at `POST /query` in `apps/server/src/api/query.ts`, which invokes the LangGraph to get a response.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect the whole service.

- [ ] T019 [P] Implement structured logging (e.g., using Pino or Winston) throughout the application.
- [ ] T020 [P] Implement environment variable configuration (e.g., using `dotenv`) for settings like server port and database paths.
- [ ] T021 Create a `Dockerfile` for the `apps/server` to containerize the application for deployment.
- [ ] T022 Finalize the `quickstart.md` with complete and verified setup and run instructions.

---

## Dependencies & Execution Order

- **Phase 1 (Setup)** must be completed first.
- **Phase 2 (Foundational)** depends on Phase 1.
- **Phase 3 (US2)** and **Phase 4 (US1)** can start in parallel after Phase 2 is complete.
- **Phase 5 (US3)** depends on the completion of Phase 3 (knowledge ingestion) and Phase 4 (feedback mechanism).
- **Phase 6 (Polish)** can be done last.

## Implementation Strategy

### MVP First (User Stories 1 & 2)

1.  Complete Phase 1 & 2.
2.  Complete Phase 3 (Knowledge Ingestion).
3.  Complete Phase 4 (Interaction & Feedback).
4.  **STOP and VALIDATE**: At this point, the system can learn and receive feedback, which forms a viable MVP. The core data loop is complete.

### Full Feature

1.  Complete the MVP steps.
2.  Complete Phase 5 (Enhanced Responses) to make use of the collected data.
3.  Complete Phase 6 (Polish) for production readiness.
