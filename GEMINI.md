<!-- codebase-discovery:start -->
# Pragmatic Code Discovery Strategy (FFF + Fast CLI + AST Grep + AST Graph)

Use a multi-layer approach for optimal search speed and minimum token waste:

## Tier 1: Ultra-Fast File Path Search (`fff` MCP & `fd`)
Use `fff` MCP tools (or `fd`) for instant, sub-millisecond typo-tolerant file path resolution and frecency-ranked file discovery.

## Tier 2: Fast Unindexed Text Searches (`ripgrep` / `rg`, `grep_search`)
Use for:
- String literals, error messages, config keys, environment variables
- Full-text code pattern searching, non-code files (Dockerfiles, JSON, YAML)

## Tier 3: AST Syntax Matching & Structural Rewriting (`ast-grep` / `sg`)
Use `ast-grep` for language-aware AST pattern matching and structural code refactoring (ignores whitespace, comments, formatting).

## Tier 4: Deep Structural Graph Analysis (`codebase-memory-mcp`)
Use for:
- Deep multi-file call-graph tracing (`trace_path`)
- Cross-module type dependency maps (`search_graph`)
- Structural Cypher queries on established graph indices (`query_graph`)
<!-- codebase-discovery:end -->

<!-- BEGIN ENGRAM MEMORY PROTOCOL — managed by engram setup -->

## Engram Persistent Memory — Protocol

You have access to Engram, a persistent memory system that survives across sessions and compactions.

### WHEN TO SAVE (mandatory — not optional)

Call mem_save IMMEDIATELY after any of these:
- Bug fix completed
- Architecture or design decision made
- Non-obvious discovery about the codebase
- Configuration change or environment setup
- Pattern established (naming, structure, convention)
- User preference or constraint learned

Format for mem_save:
- **title**: Verb + what — short, searchable (e.g. "Fixed N+1 query in UserList", "Chose Zustand over Redux")
- **type**: bugfix | decision | architecture | discovery | pattern | config | preference
- **scope**: project (default) | personal
- **topic_key** (optional, recommended for evolving decisions): stable key like architecture/auth-model
- **content**:
  **What**: One sentence — what was done
  **Why**: What motivated it (user request, bug, performance, etc.)
  **Where**: Files or paths affected
  **Learned**: Gotchas, edge cases, things that surprised you (omit if none)

### Topic update rules (mandatory)

- Different topics must not overwrite each other (e.g. architecture vs bugfix)
- Reuse the same topic_key to update an evolving topic instead of creating new observations
- If unsure about the key, call mem_suggest_topic_key first and then reuse it
- Use mem_update when you have an exact observation ID to correct

### WHEN TO SEARCH MEMORY

When the user asks to recall something — any variation of "remember", "recall", "what did we do",
"how did we solve", "recordar", "acordate", "qué hicimos", or references to past work:
1. First call mem_context — checks recent session history (fast, cheap)
2. If not found, call mem_search with relevant keywords (FTS5 full-text search)
3. If you find a match, use mem_get_observation for full untruncated content

Also search memory PROACTIVELY when:
- Starting work on something that might have been done before
- The user mentions a topic you have no context on — check if past sessions covered it

### SESSION CLOSE PROTOCOL (mandatory)

Before ending a session or saying "done" / "listo" / "that's it", you MUST:
1. Call mem_session_summary with this structure:

## Goal
[What we were working on this session]

## Instructions
[User preferences or constraints discovered — skip if none]

## Discoveries
- [Technical findings, gotchas, non-obvious learnings]

## Accomplished
- [Completed items with key details]

## Next Steps
- [What remains to be done — for the next session]

## Relevant Files
- path/to/file — [what it does or what changed]

This is NOT optional. If you skip this, the next session starts blind.

### PASSIVE CAPTURE — automatic learning extraction

When completing a task or subtask, include a "## Key Learnings:" section at the end of your response
with numbered items. Engram will automatically extract and save these as observations.

Example:
## Key Learnings:

1. bcrypt cost=12 is the right balance for our server performance
2. JWT refresh tokens need atomic rotation to prevent race conditions

You can also call mem_capture_passive(content) directly with any text that contains a learning section.
This is a safety net — it captures knowledge even if you forget to call mem_save explicitly.

### AFTER COMPACTION

If you see a message about compaction or context reset, or if you see "FIRST ACTION REQUIRED" in your context:
1. IMMEDIATELY call mem_session_summary with the compacted summary content — this persists what was done before compaction
2. Then call mem_context to recover any additional context from previous sessions
3. Only THEN continue working

Do not skip step 1. Without it, everything done before compaction is lost from memory.

<!-- END ENGRAM MEMORY PROTOCOL -->

<!-- BEGIN KARPATHY LLM CODING GUIDELINES -->
# Global Behavioral Guidelines (Karpathy LLM Rules)

Behavioral guidelines to reduce common LLM coding mistakes.

## 1. Think Before Coding
**Don't assume. Don't hide confusion. Surface tradeoffs.**
Before implementing:
- **Zero Sycophancy / No Sugarcoating**: Never flatter, appease, or blindly agree with the user's wishes. You are an equal pair-programming partner, NOT a submissive assistant. Both you and the user can be wrong. State the honest technical truth, surface flaws, and push back directly against bad or over-engineered ideas.
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First
**Minimum code that solves the problem. Nothing speculative.**
- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.
Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes
**Touch only what you must. Clean up only your own mess.**
When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.
When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.
The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution
**Define success criteria. Loop until verified.**
Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"
For multi-step tasks, state a brief plan:
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
<!-- END KARPATHY LLM CODING GUIDELINES -->

<!-- BEGIN MATT POCOCK SKILLS AUTO-INTEGRATION PROTOCOL -->
# Auto-Integration Protocol: Matt Pocock Engineering Skills + Core Stack

Whenever ANY Matt Pocock engineering skill (`research`, `wayfinder`, `domain-modeling`, `to-tickets`, `to-spec`, `grilling`, `tdd`, `code-review`, `diagnosing-bugs`, `codebase-design`) is invoked or active, ALWAYS automatically apply these core stack integrations:

1. **Auto-Codebase-Memory-MCP Graph**: During `wayfinder`, `research`, or `code-review`, ALWAYS use `codebase-memory-mcp` (`search_graph`, `trace_path`, `get_code_snippet`) for code discovery rather than raw file grepping.
2. **Auto-Engram Persistence**:
   - Immediately save key findings from `research` or `domain-modeling` via `mem_save(type="architecture"|"decision")`.
   - Log bug root causes from `diagnosing-bugs` via `mem_save(type="bugfix")`.
   - Update `mem_session_summary` upon completing tasks or ticket streams.
3. **Auto-Ponytail & Caveman Ultra**:
   - In `implement` and `tdd`, enforce YAGNI, code-first diffs, and 1-liner preference.
   - Compress responses with `Caveman Ultra` (abbreviations, no filler, `X → Y` causality).
4. **Auto-Karpathy Surgical Rules**:
   - Surface assumptions before `to-spec` or `grilling`.
   - Touch only necessary lines during `implement` / `tdd`.
<!-- END MATT POCOCK SKILLS AUTO-INTEGRATION PROTOCOL -->

<!-- BEGIN STRICT EVIDENCE RULES -->
# Strict Evidence Rules (No Assumptions Policy)

NEVER attempt a code modification based on unverified assumptions. Every edit MUST be backed by at least one piece of empirical evidence:
1. Failing test execution output
2. Exact stack trace or log error
3. Compiler or typechecker error output
4. Verified search result or primary documentation fact

If no evidence exists: RUN SEARCH FIRST. If still unavailable, ASK THE USER.
<!-- END STRICT EVIDENCE RULES -->

<!-- BEGIN AUTOMATED POST-EDIT VERIFICATION -->
# Automated Post-Edit Verification Protocol

After making any code modification, ALWAYS run the project's automated verification suite before declaring completion:
1. Format & Lint (`eslint`, `biome`, `ruff`, `golangci-lint`, `cargo clippy`)
2. Typecheck (`tsc`, `mypy`, `pyright`)
3. Unit & Integration Tests (`vitest`, `jest`, `pytest`, `go test`, `cargo test`)
<!-- END AUTOMATED POST-EDIT VERIFICATION -->

<!-- BEGIN HARDENED FILE EDITING PROTOCOL -->
# Hardened File Editing & Anti-Leak Protocol

1. **`write_to_file` Strict Scope**: Use ONLY for creating NEW, non-existent files.
2. **Existing File Edits**: NEVER use `write_to_file` to edit an existing file. Use `replace_file_content` (single contiguous block) or `multi_replace_file_content` (multi-chunk edits).
3. **No Whole-File Rewrites**: Replacing 5 lines in a 300-line file must emit only the 10-line diff chunk.
4. **No Symptom Masking**: Never catch exceptions silently or return dummy fallbacks to pass tests. Fix the true root cause.
5. **No False Declarations**: NEVER claim a task is completed until the automated verification suite passes with exit code 0.
<!-- END HARDENED FILE EDITING PROTOCOL -->

<!-- BEGIN COMMAND OUTPUT COMPRESSION (RTK) -->
# Command Output Compression Protocol (RTK Integration)

1. **Token-Saving CLI Execution**: When running shell commands (`npm test`, `pytest`, `cargo test`, `git status`, `git log`, `docker ps`), leverage `rtk` (Rust Token Killer) or filter verbose outputs to strip boilerplate log noise, progress bars, and passing assertions.
2. **Preserve Signal**: Always preserve exact failing stack traces, compiler line numbers, and error diagnostics needed for debugging.
<!-- END COMMAND OUTPUT COMPRESSION (RTK) -->

<!-- BEGIN EXTREME CONCISION & TELEGRAPHIC OUTPUT PROTOCOL -->
# Extreme Concision & Telegraphic Output Protocol

When reporting information, summaries, or results:
1. **Sacrifice Grammar for Speed & Tokens**: Omit articles (a/an/the), filler words, pleasantries, hedging, and complete sentence structures.
2. **Telegraphic & Dense Format**: Use raw technical fragments, bulleted points, and causality arrows (`X → Y`).
3. **Zero Loss of Technical Precision**: Keep file links, line numbers, error traces, and exact variable names 100% intact while stripping prose.
<!-- END EXTREME CONCISION & TELEGRAPHIC OUTPUT PROTOCOL -->




