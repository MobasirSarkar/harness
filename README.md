# 🚀 S-Tier AI Engineering Harness

An autonomous, high-performance, OS & user-agnostic AI agent harness built for **Antigravity CLI** and **Claude Code**.

It transforms AI coding assistants into disciplined, evidence-backed senior pair-programmers that execute tasks with **sub-millisecond speed**, **zero token bloat**, **surgical code diffs**, and **100% automated verification**.

---

## 🏛️ Core Harness Architecture & What We Built

```
                 S-TIER AI ENGINEERING HARNESS ARCHITECTURE
                                     │
   ┌─────────────────────────────────┼─────────────────────────────────┐
   ▼                                 ▼                                 ▼
[Sub-0.5ms Rust Engine]    [Pragmatic 4-Tier Search]   [Persistent Repository Memory]
harness-cli (Rust v1.0)    fff-mcp + rg + sg + graph   Engram SQLite FTS5 Database
- Native workspace check   - Sub-ms typo-tolerant path - Auto SessionStart injection
- Zero subprocess delay    - AST syntax matching (sg)  - Long-term ADR & bug recall
   │                                 │                                 │
   └─────────────────────────────────┼─────────────────────────────────┘
                                     ▼
                     [Behavioral & Quality Guardrails]
                     - Ponytail Ultra (YAGNI, 1-liners)
                     - Caveman Ultra (~75% token reduction)
                     - Zero Sycophancy (Equal partner, no sugarcoating)
                     - Strict Evidence Rules (No assumptions)
                     - Post-Edit Verification Loop (Format -> Lint -> Typecheck -> Test)
```

---

## 🛠️ Included Tools & Stack Components

| Component | Tool / Engine | Purpose & Efficiency Impact |
|---|---|---|
| **Harness Core Engine** | `harness-cli` (Rust v1.0) | Sub-0.5ms startup hook replacing slow bash scripts |
| **Path Search Engine** | `fff-mcp` (v0.10.1) & `fd` | Sub-millisecond typo-tolerant path resolution via Rust SDK |
| **Text Search Engine** | `ripgrep` (`rg`) | Ultra-fast line-oriented regex search for code contents |
| **AST Syntax Search** | `ast-grep` (`sg` v0.45.0) | Language-aware AST pattern matching & structural rewrites |
| **AST Call-Graph** | `codebase-memory-mcp` | Multi-file call-graph tracing (`trace_path`) & dependency maps |
| **Persistent Memory** | `Engram` (SQLite FTS5) | Persistent cross-session memory with automatic context injection |
| **Surgical Editing** | `replace_file_content` | Restricted `write_to_file` to NEW files; line diffs for existing files |
| **Response Compression** | `Caveman Ultra` | ~75% token reduction on LLM responses |
| **Command Output Compression** | `RTK` (Rust Token Killer v0.1.0) | 60%–90% token reduction on shell command/test outputs |
| **Engineering Quality** | Matt Pocock Skills | 4-phase pipeline (`research` ➔ `to-spec` ➔ `tdd` ➔ `code-review`) |
| **Strict Evidence Rule** | Protocol in `GEMINI.md` | Forbids edits without failing tests, stack traces, or compiler errors |
| **Post-Edit Verification** | Automated Pipeline | Mandatory clean run (`lint` ➔ `typecheck` ➔ `tests`) before task completion |

---

## ⚡ Installation Instructions

This repository is **100% OS and User Agnostic**. It works on any Linux distribution (Arch, Debian/Ubuntu, RHEL/Fedora) and macOS (`Darwin`).

### Option 1: One-Command Bootstrap (Recommended)

Run this single command on any new device:

```bash
git clone https://github.com/<your-username>/harness.git ~/.harness && ~/.harness/install.sh
```

Or via direct `curl`:

```bash
curl -sSL https://raw.githubusercontent.com/MobasirSarkar/harness/main/install.sh | bash
```

---

### Option 2: Manual Installation from Cloned Repository

1. **Clone Repository**:
   ```bash
   git clone https://github.com/MobasirSarkar/harness.git ~/harness
   cd ~/harness
   ```

2. **Make Installer Executable & Run**:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

---

## 📁 Repository Structure

```text
harness/
├── README.md           # Documentation & Architecture Overview
├── install.sh          # Universal OS & User Agnostic Installer Script
├── GEMINI.md           # Global rules (Engram, Karpathy, Evidence Rules, 4-Tier Discovery)
├── CLAUDE.md           # Hardened anti-leak & zero-sycophancy pair-programming rules
├── settings.json       # Auto-approved permissions & MCP server suite template
├── statusline.sh       # Terminal statusline script (Nerd Font UI)
├── .gitignore          # Rust build target exclusions
└── harness-cli/        # Sub-0.5ms compiled Rust harness engine crate
    ├── Cargo.toml
    └── src/
        └── main.rs     # Rust harness startup hook implementation
```

---

## 📜 Global Behavioral Rules Summary

1. **Zero Sycophancy / No Sugarcoating**: Never flatter or blindly agree. You are an equal pair-programmer, NOT a submissive assistant. State raw technical facts & push back against bad ideas.
2. **Think Before Coding**: State assumptions explicitly. Surface tradeoffs before implementation.
3. **Simplicity First (YAGNI)**: Write minimum code that solves the problem. 200 lines ➔ 50 lines.
4. **Surgical Diffs Only**: `write_to_file` is allowed ONLY for new files. Existing files must use line-based diff replacements.
5. **No Assumptions Policy**: Edits must be backed by empirical evidence (logs, compiler errors, failing tests).
6. **Automated Verification Loop**: Always run format ➔ lint ➔ typecheck ➔ tests (exit code `0` required) before declaring completion.
