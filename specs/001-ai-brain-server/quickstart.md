# Quickstart for AI Brain Server

This guide provides instructions to set up and run the AI Brain Server locally for development and testing.

## Prerequisites

- Node.js (v20 or later)
- Rust (latest stable toolchain)
- pnpm (or your preferred package manager)
- OpenAI API Key (set as an environment variable `OPENAI_API_KEY`)

## Setup

1.  **Navigate to the project root**:
    ```bash
    cd /path/to/z-project
    ```

2.  **Install all workspace dependencies**:
    ```bash
    pnpm install
    ```

3.  **Build the Rust component**:
    The Rust component (a native N-API addon) needs to be built before the server can run.
    ```bash
    pnpm --filter rust_components build
    ```

4.  **Create `.env` file**:
    Create a `.env` file in the `apps/server` directory and add your `OPENAI_API_KEY`.
    ```
    # apps/server/.env
    OPENAI_API_KEY=sk-your-openai-api-key
    ```
    *(Note: Refer to `apps/server/.env.example` for the format.)*

## Running the Server

Once the setup is complete, you can start the development server:

```bash
# From the project root, targeting the server app
pnpm --filter @z-project/server dev
```

The server will start (by default on port `3000`) and will have access to the compiled Rust module.

## Local Vector Store

- **Database Location**: The FAISS vector store will be saved to a local file within the `apps/server` directory (e.g., `apps/server/db/faiss_index`). This file will be created automatically on the first run.
- **Persistence**: The database is persisted on disk, so the server can be restarted without losing the ingested knowledge. The database file path is already in `.gitignore`.

## Testing

To run the test suite:

```bash
# From the project root

# Run TypeScript tests for the server
pnpm --filter @z-project/server test

# Run Rust tests for the component
pnpm --filter rust_components test
```
*(Note: Ensure you are in the project root when running these commands.)*