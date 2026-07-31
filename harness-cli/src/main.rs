mod tools;
mod auditor;
mod providers;
mod engine;

use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let is_json = env::args().any(|arg| arg == "--json");
    let mut raw_buf = Vec::new();
    let mut handle: Box<dyn Write> = if is_json {
        Box::new(&mut raw_buf)
    } else {
        Box::new(io::BufWriter::new(io::stdout().lock()))
    };

    // 1. Header & 4-Tier Discovery Strategy
    writeln!(handle, "=== S-TIER AI ENGINEERING HARNESS ACTIVE (Rust Engine v1.0) ===")?;
    writeln!(handle, "Code discovery strategy: Use fff MCP / fd for instant sub-ms file path resolution. Use ripgrep (rg) for text searching. Use ast-grep (sg) for AST syntax matching & structural rewrites. Reserve codebase-memory-mcp (search_graph, trace_path) for deep structural AST call-graph queries.")?;

    // 2. Behavioral Modes & Zero-Sycophancy Rules
    writeln!(handle, "\nPONYTAIL MODE ACTIVE — level: ultra")?;
    writeln!(handle, "You are a lazy senior developer (ultra mode). YAGNI rule active.")?;
    writeln!(handle, "- Before any code: does it need to exist at all? Standard lib? Native platform feature? Can it be 1 line?")?;
    writeln!(handle, "- Write minimum code that works. No unrequested abstractions, no avoidable dependencies, no boilerplate.")?;
    writeln!(handle, "- Code/diffs first. Deletion over addition. If explanation longer than code, delete explanation.")?;

    writeln!(handle, "\nCAVEMAN MODE ACTIVE — level: ultra")?;
    writeln!(handle, "Ultra-compressed communication mode. ACTIVE EVERY RESPONSE.")?;
    writeln!(handle, "- Drop: articles (a/an/the), filler (just/really/basically/actually), pleasantries (sure/happy to), hedging, conjunctions.")?;
    writeln!(handle, "- Abbreviate aggressively: DB, auth, config, req, res, fn, impl, obj, prop, etc.")?;
    writeln!(handle, "- Use arrows for causality (X → Y). Use short fragments. Keep full technical accuracy.")?;

    writeln!(handle, "\nKARPATHY & EQUAL PARTNERSHIP RULES ACTIVE")?;
    writeln!(handle, "1. Zero Sycophancy / No Sugarcoating: Never flatter, appease, or blindly agree. You are an equal pair-programmer, NOT a submissive assistant. Both can be wrong. State raw technical facts & push back directly against bad/over-engineered ideas.")?;
    writeln!(handle, "2. Think Before Coding: Don't assume, surface tradeoffs, ask if ambiguous.")?;
    writeln!(handle, "3. Simplicity First: Minimum code, no speculative features/abstractions. 200 lines -> 50 lines.")?;
    writeln!(handle, "4. Surgical Changes & Anti-Leak Rules: write_to_file is restricted ONLY to NEW files. Edit existing files strictly using replace_file_content or multi_replace_file_content.")?;

    writeln!(handle, "\nRTK COMMAND OUTPUT COMPRESSION ACTIVE")?;
    writeln!(handle, "- Shell command execution (rtk/cargo/npm/git/pytest) outputs are compressed (60-90% token reduction). Boilerplate log noise is collapsed while preserving exact stack traces and error signals.")?;

    writeln!(handle, "\nMATT POCOCK SKILLS AUTO-INTEGRATION ACTIVE")?;
    writeln!(handle, "- Automatically pairs Matt's engineering skills (research, wayfinder, tdd, code-review, etc.) with Modern Fast Search (fff/rg/fd/ast-grep), Codebase Memory MCP, Engram persistence, Ponytail/Caveman compression, RTK output filtering, and Karpathy surgical rules.")?;

    writeln!(handle, "\nSTRICT EVIDENCE & VERIFICATION RULES ACTIVE")?;
    writeln!(handle, "- No Assumptions Policy: Edits must be backed by logs, compiler errors, failing tests, or verified search proof.")?;
    writeln!(handle, "- Post-Edit Verification: Always run format -> lint -> typecheck -> test (exit code 0 required) before completing task.")?;

    // 3. Fast Workspace / Git Status Inspection (Microsecond Execution)
    if let Ok(cwd) = env::current_dir() {
        if let Ok(git_branch) = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&cwd)
            .output()
        {
            if git_branch.status.success() {
                let branch_name = String::from_utf8_lossy(&git_branch.stdout).trim().to_string();
                writeln!(handle, "\n=== ACTIVE WORKSPACE GIT CONTEXT ===")?;
                writeln!(handle, "Branch: {}", branch_name)?;

                if let Ok(status) = Command::new("git")
                    .args(["status", "--short"])
                    .current_dir(&cwd)
                    .output()
                {
                    if status.status.success() && !status.stdout.is_empty() {
                        let status_str = String::from_utf8_lossy(&status.stdout);
                        let modified_count = status_str.lines().count();
                        writeln!(handle, "Modified Files ({}):", modified_count)?;
                        for line in status_str.lines().take(5) {
                            writeln!(handle, "  {}", line)?;
                        }
                        if modified_count > 5 {
                            writeln!(handle, "  ... and {} more files", modified_count - 5)?;
                        }
                    }
                }
            }
        }
    }

    // 4. Fast Engram Persistent Memory Query
    writeln!(handle, "\n=== ENGRAM PERSISTENT MEMORY CONTEXT ===")?;
    if let Ok(engram_output) = Command::new("/home/mobasir/go/bin/engram")
        .arg("context")
        .output()
    {
        if engram_output.status.success() {
            handle.write_all(&engram_output.stdout)?;
        }
    }

    handle.flush()?;
    drop(handle);

    if is_json {
        let text = String::from_utf8_lossy(&raw_buf);
        let json_escaped = text
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        print!("{{\"additionalContext\":\"{}\"}}", json_escaped);
    }

    Ok(())
}
