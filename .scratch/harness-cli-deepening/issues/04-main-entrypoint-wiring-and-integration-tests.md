# 04 — Wire main.rs Entrypoint & Add End-to-End Integration Suite

**What to build:** 10-line clean `main.rs` CLI dispatcher supporting `--json` hook output and `--audit` diagnostic report modes. Includes full end-to-end CLI integration test suite.

**Blocked by:** 03 — Implement Structured OutputFormatter Adapter

**Status:** done

- [x] Refactor `main.rs` to use `HarnessEngine` and `OutputFormatter`
- [x] Add support for `--audit` command line flag for explicit tool diagnosis report
- [x] Ensure backward compatibility with `--json` and standard terminal output
- [x] Rebuild release binary and verify with `install.sh`
- [x] Add integration tests in `tests/cli_tests.rs`
