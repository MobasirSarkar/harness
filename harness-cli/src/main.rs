mod auditor;
mod engine;
mod formatter;
mod providers;
mod tools;

use engine::{EngineOpts, HarnessEngine};
use formatter::OutputFormatter;
use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_json = args.iter().any(|arg| arg == "--json");
    let is_audit = args.iter().any(|arg| arg == "--audit");
    let is_short = args.iter().any(|arg| arg == "--status-short" || arg == "--short");

    if is_short {
        let auditor = auditor::ToolAuditor::default_system();
        println!("{}", auditor.generate_short_status_text());
        return Ok(());
    }

    let opts = EngineOpts::default();
    let ctx = HarnessEngine::compose_context(&opts);

    if is_audit {
        let report = OutputFormatter::to_audit_report(&ctx);
        println!("{}", report);
    } else if is_json {
        let json_payload = OutputFormatter::to_json(&ctx);
        print!("{}", json_payload);
    } else {
        let text = OutputFormatter::to_text(&ctx);
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", text)?;
        handle.flush()?;
    }

    Ok(())
}
