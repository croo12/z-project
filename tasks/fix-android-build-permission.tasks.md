# Tasks: Fix Android Build Permission Error

- [x] Clean Build Artifacts <!-- id: 0 -->
    - [x] `rm -rf apps/mobile/src-tauri/target`
    - [x] `rm -rf apps/mobile/src-tauri/gen`
- [x] Regenerate Capabilities <!-- id: 1 -->
    - [x] Run `pnpm tauri android build` (or `tauri build --target android`) to regenerate files (can be stopped once generation is done, or let it fail if well gen files are created)ct, as long as gen files are created)
- [x] Verify Fix <!-- id: 2 -->
    - [x] `git commit -m "feat: mcp 설치"` should succeed
