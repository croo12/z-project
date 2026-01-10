# Research & Decisions for AI Brain Server

This document outlines the key technical decisions made during the planning phase, based on the requirements from the feature specification and the initial technology choices.

## 1. Vector Database: FAISS

- **Decision**: Use a local, file-based FAISS vector store, accessed via `langchain/faiss-node`.
- **Rationale**:
    - **Simplicity**: For initial development and a personal knowledge base, a file-based solution is the simplest to set up and manage. It requires no external services or daemons.
    - **Cost-Effective**: There are no costs associated with a local file-based store.
    - **Future-Proof**: LangChain's `VectorStore` abstraction makes it straightforward to replace FAISS with a more robust, production-grade database like ChromaDB, Weaviate, or Pinecone in the future without major architectural changes. The core logic will remain the same.
- **Alternatives Considered**:
    - **Managed Services (Pinecone)**: Rejected for now due to the added complexity and cost for a personal project.
    - **Self-Hosted (ChromaDB)**: A strong candidate for future versions, but overkill for the initial MVP.

## 2. Rust and Node.js Integration

- **Decision**: Integrate the Rust component as a native Node.js addon using **N-API**.
- **Rationale**:
    - **Performance**: For a "bit of Rust" intended for performance-critical tasks (e.g., complex data processing, specialized algorithms), direct in-process calls via N-API offer the highest performance with the lowest latency, avoiding network overhead.
    - **Development Complexity**: While there is a learning curve for N-API, it provides a very clean and tight integration. It avoids the need to manage a separate microservice, its deployment, and its communication protocol.
    - **Build Process**: The build process can be integrated into the main `apps/server` `package.json` scripts, making it relatively seamless for development and CI/CD.
- **Alternatives Considered**:
    - **Separate Microservice**: Rejected for now. The overhead of managing a separate service (API definitions, deployment, containerization, inter-service communication) is too high for the initial "small" scope of the Rust component.
    - **WebAssembly (WASM)**: A viable alternative, but N-API provides better access to system-level resources if needed and can be more performant for heavy, synchronous computations. The toolchain for N-API with Rust (`napi-rs`) is also very mature.

## 3. Testing Framework: Vitest

- **Decision**: Use **Vitest** for all TypeScript testing in the `apps/server` project.
- **Rationale**:
    - **Speed and Simplicity**: Vitest is known for its high performance and simple configuration, especially in modern ESM-based projects.
    - **Modern Features**: It offers a great developer experience with features like a watch mode, an interactive UI, and compatibility with Vite's ecosystem.
    - **Compatibility**: It is fully compatible with Jest's API, making migration or adoption easy.
- **Alternatives Considered**:
    - **Jest**: A solid and mature choice, but Vitest is generally considered more modern and faster for new projects. Given this is a new server application, starting with Vitest is a good forward-looking decision.
