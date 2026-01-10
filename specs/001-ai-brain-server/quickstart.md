# Quickstart for AI Brain Server

This guide provides instructions to set up and run the AI Brain Server locally for development and testing.

## Prerequisites

- Node.js (v20 or later)
- Rust (latest stable toolchain)
- pnpm (or your preferred package manager)

## Setup

1.  **Navigate to the server directory**:
    ```bash
    cd apps/server
    ```

2.  **Install TypeScript dependencies**:
    ```bash
    pnpm install
    ```

3.  **Build the Rust component**:
    The Rust component (a native N-API addon) needs to be built before the server can run.
    ```bash
    cd rust_components
    # This command will build the .node file and place it in the correct directory
    pnpm build 
    cd .. 
    ```
    *(Note: The `build` script in `rust_components/package.json` will need to be configured to handle the N-API build process).*

## Running the Server

Once the setup is complete, you can start the development server:

```bash
# From the apps/server directory
pnpm dev
```

The server will start (by default on a port like `3000`) and will have access to the compiled Rust module.

## Local Vector Store

- **Database Location**: The FAISS vector store will be saved to a local file within the `apps/server` directory (e.g., `apps/server/db/faiss.index`). This file will be created automatically on the first run.
- **Persistence**: The database is persisted on disk, so the server can be restarted without losing the ingested knowledge. It's recommended to add the database file path to `.gitignore`.

## Testing

To run the test suite:

```bash
# From the apps/server directory

# Run TypeScript tests
pnpm test:ts

# Run Rust tests
pnpm test:rust
```
*(Note: These test scripts will need to be defined in `apps/server/package.json` to execute Vitest and `cargo test` respectively).*
