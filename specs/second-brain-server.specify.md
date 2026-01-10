# Specification

## Requirements


# Specification: Second Brain Server for AI Workflow Automation

## 1. Vision
To create a personalized "Second Brain" server that empowers a developer's AI-driven workflow. This server will act as a central hub for managing context, enforcing standards, and orchestrating complex development tasks for an AI agent, moving beyond generic LLM interactions to produce production-quality results consistently.

## 2. Problems to Solve
1.  **Context Repetition:** The need to manually provide coding standards, architectural patterns, and project-specific information for every AI request is inefficient and error-prone.
2.  **Lack of Composability:** Different projects require different contexts (tech stacks, domains, design systems). A generic AI cannot easily adapt.
3.  **Manual Verification:** AI-generated code requires manual verification (linting, testing, building), breaking the automation flow.
4.  **Procedural Gap:** Simple prompts are insufficient for production-level tasks. A more rigorous, multi-step process is needed to guide the AI.

## 3. Core Requirements & Features

### 3.1. Context Hub (RAG-based)
- The server must implement a RAG (Retrieval-Augmented Generation) pipeline to act as a "Context Hub".
- It shall store, manage, and retrieve various types of context, including:
    - **Constitution:** Core coding principles, style guides, and architectural rules.
    - **Project-Specific Knowledge:** Documentation, technology stack details, and domain-specific information for different projects.
    - **Code Patterns:** Frequently used code snippets and patterns.
- It must provide an internal API for the Workflow Engine to retrieve the most relevant context for any given task.

### 3.2. Workflow Engine (LangGraph-based)
- The server must use a graph-based framework like LangGraph to define and execute stateful, potentially cyclical AI workflows.
- This allows for creating complex agentic behaviors beyond simple prompt-response chains.
- **Example Workflow:**
    1.  Receive a task from a client (e.g., "create a new React component").
    2.  **Retrieve Context:** Use the RAG pipeline to fetch relevant coding principles and component examples.
    3.  **Generate Code:** Prompt the LLM with the task and retrieved context to generate the initial code.
    4.  **Verify:** Automatically run `lint` and `test` commands on the generated code.
    5.  **Self-Correction Loop:** If verification fails, feed the error messages back to the LLM for automated correction. This loop continues until the code passes all checks.
    6.  **Complete:** Once verified, report the success back to the client.

### 3.3. API Layer
- The server will expose a clear API for client applications (`apps/web`, `apps/mobile`, IDE extensions) to interact with.
- Initial endpoints should include:
    - An endpoint to manage the "Constitution" (CRUD for principles).
    - An endpoint to trigger an AI workflow with a specific task.

## 4. Technology Stack
- **Language:** TypeScript
- **Runtime:** Node.js
- **Core AI Libraries:**
    - `LangChain.js` (for overall structure and integrations)
    - `@langchain/langgraph` (for the workflow engine)
- **RAG Components:**
    - A vector store library compatible with `LangChain.js` (e.g., in-memory for PoC, scalable solution for production).
    - An embedding model provider.
- **LLM:** An LLM provider integrated via `LangChain.js` (e.g., OpenAI, Anthropic, Gemini).


## User Stories

As a developer, I want to define my coding principles and project contexts in a central server, so that my AI agent can automatically access this knowledge to generate high-quality, consistent code without me having to repeat myself.
