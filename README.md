# My Rust Android App

This is a Tauri v2 application built with Rust (Backend) and Vanilla JS (Frontend).

## Features
- **News**: RSS-based news feed with keyword-based categorization.
- **Todos**: Manage your daily tasks.
- **Work Log**: Track your work hours.
- **Android Support**: Fully compatible with Android devices via Tauri v2.

## Prerequisites
- **Rust**: Standard installation.
- **Java JDK 17**: Required for Android build.
- **Android Studio & SDK**: Required for Android build.
- **Node.js**: For package management.

## Getting Started

### Install Dependencies
```bash
npm install
```

### Run on Desktop
```bash
npm run tauri dev
```

### Run on Android
First, ensure Android Studio is installed and `ANDROID_HOME` environment variable is set.

1. Initialize Android project (first time only):
   ```bash
   npm run tauri android init
   ```
2. Run on device/emulator:
   ```bash
   npm run tauri android dev
   ```
   ```
   
## Development Workflow

### Code Quality

This project uses **Husky** to enforce quality checks:

- **Pre-commit**: Automatically runs `ESLint` and `Prettier` on staged files.
- **Pre-push**: Runs all unit tests via `pnpm test`.

To manually run tests:
```bash
pnpm test
```
To run tests in a specific package:
```bash
pnpm --filter <package-name> test
```
