use std::env;
use std::process::Command;
use crate::auditor::{PathResolver, SystemPathResolver, ToolAuditor};

pub trait CommandExecutor {
    fn execute(&self, cmd: &str, args: &[&str]) -> Option<String>;
}

pub struct SystemCommandExecutor;

impl CommandExecutor for SystemCommandExecutor {
    fn execute(&self, cmd: &str, args: &[&str]) -> Option<String> {
        let cwd = env::current_dir().ok()?;
        let output = Command::new(cmd).args(args).current_dir(cwd).output().ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HarnessContext {
    pub header: String,
    pub rules: Vec<String>,
    pub git_context: Option<String>,
    pub engram_context: Option<String>,
    pub tool_telemetry: String,
}

pub trait ContextProvider {
    fn provide(&self, context: &mut HarnessContext);
}

pub struct RulesProvider;

impl ContextProvider for RulesProvider {
    fn provide(&self, context: &mut HarnessContext) {
        context.header = "=== S-TIER AI ENGINEERING HARNESS ACTIVE (Rust Engine v2.0) ===".to_string();
        
        let rules = vec![
            "Code discovery strategy: Use fff MCP / fd for instant sub-ms file path resolution. Use ripgrep (rg) for text searching. Use ast-grep (sg) for AST syntax matching & structural rewrites. Reserve codebase-memory-mcp for deep structural call-graph queries.".to_string(),
            "PONYTAIL MODE ACTIVE — level: ultra (YAGNI rule active. Minimum code, deletion over addition).".to_string(),
            "CAVEMAN MODE ACTIVE — level: ultra (Drop filler words/articles. Use arrows X -> Y. Keep full technical accuracy).".to_string(),
            "KARPATHY & EQUAL PARTNERSHIP RULES ACTIVE (Zero sycophancy. Surgical edits: write_to_file ONLY for NEW files, replace_file_content for existing).".to_string(),
            "RTK COMMAND OUTPUT COMPRESSION ACTIVE (60-90% token reduction for CLI output logs).".to_string(),
            "MATT POCOCK SKILLS AUTO-INTEGRATION ACTIVE (Paired with Fast Search, Codebase Memory, Engram, RTK & Surgical Rules).".to_string(),
            "STRICT EVIDENCE & VERIFICATION RULES ACTIVE (No assumptions. Post-edit lint -> typecheck -> test verification).".to_string(),
        ];
        
        context.rules = rules;
    }
}

pub struct GitProvider<E: CommandExecutor> {
    executor: E,
}

impl<E: CommandExecutor> GitProvider<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl ContextProvider for GitProvider<SystemCommandExecutor> {
    fn provide(&self, context: &mut HarnessContext) {
        self.provide_with_executor(context);
    }
}

impl<E: CommandExecutor> GitProvider<E> {
    pub fn provide_with_executor(&self, context: &mut HarnessContext) {
        if let Some(branch) = self.executor.execute("git", &["rev-parse", "--abbrev-ref", "HEAD"]) {
            let mut git_out = format!("Branch: {}\n", branch);
            if let Some(status) = self.executor.execute("git", &["status", "--short"]) {
                if !status.is_empty() {
                    let lines: Vec<&str> = status.lines().collect();
                    git_out.push_str(&format!("Modified Files ({}):\n", lines.len()));
                    for line in lines.iter().take(5) {
                        git_out.push_str(&format!("  {}\n", line));
                    }
                    if lines.len() > 5 {
                        git_out.push_str(&format!("  ... and {} more files\n", lines.len() - 5));
                    }
                }
            }
            context.git_context = Some(git_out);
        }
    }
}

pub struct EngramProvider<E: CommandExecutor> {
    executor: E,
}

impl<E: CommandExecutor> EngramProvider<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl ContextProvider for EngramProvider<SystemCommandExecutor> {
    fn provide(&self, context: &mut HarnessContext) {
        self.provide_with_executor(context);
    }
}

impl<E: CommandExecutor> EngramProvider<E> {
    pub fn provide_with_executor(&self, context: &mut HarnessContext) {
        let home = env::var("HOME").unwrap_or_default();
        let engram_bin = format!("{}/go/bin/engram", home);
        if let Some(ctx) = self.executor.execute(&engram_bin, &["context"]) {
            if !ctx.is_empty() {
                context.engram_context = Some(ctx);
            }
        }
    }
}

pub struct ToolStatusProvider<R: PathResolver> {
    auditor: ToolAuditor<R>,
}

impl ToolStatusProvider<SystemPathResolver> {
    pub fn default_system() -> Self {
        Self {
            auditor: ToolAuditor::default_system(),
        }
    }
}

impl<R: PathResolver> ToolStatusProvider<R> {
    pub fn new(auditor: ToolAuditor<R>) -> Self {
        Self { auditor }
    }
}

impl<R: PathResolver> ContextProvider for ToolStatusProvider<R> {
    fn provide(&self, context: &mut HarnessContext) {
        context.tool_telemetry = self.auditor.generate_summary_text();
    }
}
