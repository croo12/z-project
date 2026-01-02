## 2025-10-28 - [Parallel RSS Fetching]
**Learning:** Sequential HTTP requests in a loop are a major bottleneck, even with async. Using `tauri::async_runtime::spawn` (or `tokio::spawn`) with a shared `reqwest::Client` allows for concurrent fetching, drastically reducing total wait time. Also, `reqwest::Client` is `Arc` internally, so it's cheap to clone and share.
**Action:** When fetching multiple resources, always spawn concurrent tasks and reuse the HTTP client.
