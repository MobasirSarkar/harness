use serde_json::json;
use crate::providers::HarnessContext;

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn to_text(ctx: &HarnessContext) -> String {
        let mut out = String::new();
        out.push_str(&ctx.header);
        out.push('\n');

        for rule in &ctx.rules {
            out.push_str("\n- ");
            out.push_str(rule);
        }

        out.push_str("\n\n");
        out.push_str(&ctx.tool_telemetry);

        if let Some(ref git) = ctx.git_context {
            out.push_str("\n=== ACTIVE WORKSPACE GIT CONTEXT ===\n");
            out.push_str(git);
        }

        if let Some(ref engram) = ctx.engram_context {
            out.push_str("\n=== ENGRAM PERSISTENT MEMORY CONTEXT ===\n");
            out.push_str(engram);
            out.push('\n');
        }

        out
    }

    pub fn to_json(ctx: &HarnessContext) -> String {
        let raw_text = Self::to_text(ctx);
        let payload = json!({
            "additionalContext": raw_text
        });
        serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn to_audit_report(ctx: &HarnessContext) -> String {
        let mut out = String::new();
        out.push_str(&ctx.header);
        out.push_str("\n\n");
        out.push_str(&ctx.tool_telemetry);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_valid_schema() {
        let mut ctx = HarnessContext::default();
        ctx.header = "=== TEST HEADER ===".to_string();
        ctx.rules = vec!["Rule 1".to_string(), "Rule 2 with \"quotes\" & \n newlines".to_string()];
        ctx.tool_telemetry = "Tool Telemetry".to_string();

        let json_str = OutputFormatter::to_json(&ctx);
        let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("Must be valid JSON");

        assert!(parsed.get("additionalContext").is_some());
        let text = parsed["additionalContext"].as_str().unwrap();
        assert!(text.contains("=== TEST HEADER ==="));
        assert!(text.contains("Rule 2 with \"quotes\""));
    }
}
