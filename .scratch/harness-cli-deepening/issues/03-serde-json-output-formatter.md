# 03 — Implement Structured OutputFormatter Adapter

**What to build:** `OutputFormatter` (`formatter.rs`) using `serde` & `serde_json` to format `HarnessContext` into valid Antigravity hook JSON response (`{"additionalContext": "..."}`) or ANSI terminal text. Replaces manual string escaping logic.

**Blocked by:** 02 — Implement HarnessEngine & Modular Context Providers

**Status:** done

- [x] Add `serde` and `serde_json` dependencies to `Cargo.toml`
- [x] Create `OutputFormatter` struct with `to_json()` and `to_text()` methods
- [x] Ensure full compatibility with Antigravity / AGY CLI hooks (`additionalContext` JSON payload)
- [x] Add unit tests for JSON serialization and character escaping
