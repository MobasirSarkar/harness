# 01 — Create Deep ToolAuditor Module for System Tool & Telemetry Auditing

**What to build:** Standalone `ToolAuditor` (`auditor.rs` & `tools.rs`) to check PATH resolution, execution permissions, and health status for required harness CLI binaries (`ast-grep`, `fff-mcp`, `engram`, `rtk`, `rg`, `fd`). Includes fake runner adapter for fast unit testing.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] Define `HarnessTool` enum & `ToolHealth` struct representing status (`Installed`, `Missing`, `ExecutionFailed`)
- [x] Implement `ToolAuditor` with `audit_all()` and `audit_tool()` functions
- [x] Add unit test suite using fake runner / process mock
