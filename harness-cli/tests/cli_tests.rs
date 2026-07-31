use std::process::Command;

#[test]
fn test_cli_default_text_output() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_harness-cli"));
    let output = cmd.output().expect("Failed to execute harness-cli binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("=== S-TIER AI ENGINEERING HARNESS ACTIVE"));
    assert!(stdout.contains("HARNESS TOOL INVOCATION TELEMETRY & HEALTH AUDIT"));
}

#[test]
fn test_cli_json_output() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_harness-cli"));
    cmd.arg("--json");
    let output = cmd.output().expect("Failed to execute harness-cli binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("Must produce valid JSON");
    assert!(parsed.get("additionalContext").is_some());
    let ctx = parsed["additionalContext"].as_str().unwrap();
    assert!(ctx.contains("=== S-TIER AI ENGINEERING HARNESS ACTIVE"));
}

#[test]
fn test_cli_audit_flag_output() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_harness-cli"));
    cmd.arg("--audit");
    let output = cmd.output().expect("Failed to execute harness-cli binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HARNESS TOOL INVOCATION TELEMETRY & HEALTH AUDIT"));
    assert!(stdout.contains("FFF MCP Server"));
}

#[test]
fn test_cli_short_status_flag() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_harness-cli"));
    cmd.arg("--status-short");
    let output = cmd.output().expect("Failed to execute harness-cli binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HARNESS: RUST v2.0"));
    assert!(stdout.contains("ACTIVE"));
}
