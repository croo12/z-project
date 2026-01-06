use super::model::ArticleCategory;

pub const HIGH_IMPACT_KEYWORDS: &[&str] = &[
    "rust",
    "tauri",
    "react",
    "typescript",
    "javascript",
    "android",
    "kotlin",
    "webassembly",
    "wasm",
    "docker",
    "kubernetes",
    "llvm",
    "compiler",
];

pub const MEDIUM_IMPACT_KEYWORDS: &[&str] = &[
    "code",
    "programming",
    "developer",
    "api",
    "frontend",
    "backend",
    "database",
    "algorithm",
    "git",
    "linux",
    "windows",
    "macos",
    "design pattern",
    "refactoring",
];

pub const NEGATIVE_KEYWORDS: &[&str] = &[
    "stock",
    "market",
    "buffett",
    "berkshire",
    "invest",
    "politics",
    "crime",
    "murder",
    "sport",
    "celebrity",
    "gossip",
    "bitcoin",
    "crypto",
    "blockchain",
];

pub const FEEDS: &[(&str, ArticleCategory)] = &[
    // Rust
    ("https://blog.rust-lang.org/feed.xml", ArticleCategory::Rust),
    (
        "https://this-week-in-rust.org/rss.xml",
        ArticleCategory::Rust,
    ),
    // Android / Kotlin
    (
        "https://feeds.feedburner.com/blogspot/hsDu",
        ArticleCategory::Android,
    ),
    ("https://androidweekly.net/rss", ArticleCategory::Android),
    // Tauri
    ("https://tauri.app/blog/rss.xml", ArticleCategory::Tauri),
    // Web / TypeScript
    (
        "https://devblogs.microsoft.com/typescript/feed/",
        ArticleCategory::TypeScript,
    ),
    ("https://css-tricks.com/feed/", ArticleCategory::Web),
    (
        "https://www.smashingmagazine.com/feed/",
        ArticleCategory::Web,
    ),
    ("https://web.dev/feed.xml", ArticleCategory::Web),
    ("https://fettblog.eu/feed.xml", ArticleCategory::TypeScript),
    (
        "https://levelup.gitconnected.com/feed",
        ArticleCategory::Web,
    ),
    (
        "https://2ality.com/feeds/posts.xml",
        ArticleCategory::TypeScript,
    ),
    // React
    ("https://react.dev/feed.xml", ArticleCategory::React),
    ("https://overreacted.io/rss.xml", ArticleCategory::React),
    ("https://tkdodo.eu/blog/rss.xml", ArticleCategory::React),
    (
        "https://kentcdodds.com/blog/rss.xml",
        ArticleCategory::React,
    ),
    (
        "https://www.joshwcomeau.com/rss.xml",
        ArticleCategory::React,
    ),
    ("https://robinwieruch.de/index.xml", ArticleCategory::React),
    ("https://ui.dev/blog/rss", ArticleCategory::React),
    (
        "https://www.developerway.com/rss.xml",
        ArticleCategory::React,
    ),
    // AI
    ("https://openai.com/blog/rss.xml", ArticleCategory::AI),
    ("https://blogs.microsoft.com/ai/feed/", ArticleCategory::AI),
    // General / Tech
    ("https://news.ycombinator.com/rss", ArticleCategory::General),
    ("https://dev.to/feed", ArticleCategory::General),
];

pub const GEMINI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent";
