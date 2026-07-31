# 02 — Implement HarnessEngine & Modular Context Providers

**What to build:** Decoupled context composition pipeline (`engine.rs` & `providers/*`) replacing 107-line monolithic script in `main.rs`. Integrates `ToolAuditor` diagnostics into prompt injection payload. Fully testable context providers for Git status, Engram memory, and rules.

**Blocked by:** 01 — Create Deep ToolAuditor Module for System Tool & Telemetry Auditing

**Status:** ready-for-agent

- [ ] Create `ContextProvider` trait and implementations: `RulesProvider`, `GitProvider`, `EngramProvider`, `ToolStatusProvider`
- [ ] Create `HarnessEngine` to assemble providers into `HarnessContext`
- [ ] Add unit tests for context composition and fallback handling
