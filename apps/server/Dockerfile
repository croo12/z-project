# Use an official Node.js runtime as a parent image
FROM node:20-slim

# Set the working directory in the container
WORKDIR /app/apps/server

# Copy package.json and pnpm-lock.yaml for dependency installation
COPY apps/server/package.json ./package.json
COPY apps/server/pnpm-lock.yaml ./pnpm-lock.yaml
COPY pnpm-workspace.yaml /app/pnpm-workspace.yaml

# Install dependencies
RUN corepack enable
RUN pnpm install --frozen-lockfile

# Copy the rest of the application code
COPY . /app

# Build the TypeScript application
RUN pnpm --filter @z-project/server build

# Build the Rust N-API addon
RUN pnpm --filter rust_components build

# Expose the port the app runs on
EXPOSE 3000

# Run the application
CMD [ "pnpm", "--filter", "@z-project/server", "start" ]