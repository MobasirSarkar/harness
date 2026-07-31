use crate::auditor::SystemPathResolver;
use crate::providers::{
    ContextProvider, EngramProvider, GitProvider, HarnessContext, RulesProvider, SystemCommandExecutor, ToolStatusProvider,
};

pub struct EngineOpts {
    pub include_git: bool,
    pub include_engram: bool,
    pub include_tool_telemetry: bool,
}

impl Default for EngineOpts {
    fn default() -> Self {
        Self {
            include_git: true,
            include_engram: true,
            include_tool_telemetry: true,
        }
    }
}

pub struct HarnessEngine;

impl HarnessEngine {
    pub fn compose_context(opts: &EngineOpts) -> HarnessContext {
        let mut context = HarnessContext::default();

        // 1. Rules Provider
        RulesProvider.provide(&mut context);

        // 2. Git Provider
        if opts.include_git {
            let git_provider = GitProvider::new(SystemCommandExecutor);
            git_provider.provide_with_executor(&mut context);
        }

        // 3. Engram Memory Provider
        if opts.include_engram {
            let engram_provider = EngramProvider::new(SystemCommandExecutor);
            engram_provider.provide_with_executor(&mut context);
        }

        // 4. Tool Status Telemetry Provider
        if opts.include_tool_telemetry {
            let tool_provider = ToolStatusProvider::<SystemPathResolver>::default_system();
            tool_provider.provide(&mut context);
        }

        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::CommandExecutor;

    struct FakeCommandExecutor {
        git_branch: Option<String>,
        git_status: Option<String>,
    }

    impl CommandExecutor for FakeCommandExecutor {
        fn execute(&self, cmd: &str, args: &[&str]) -> Option<String> {
            if cmd == "git" && args == ["rev-parse", "--abbrev-ref", "HEAD"] {
                return self.git_branch.clone();
            }
            if cmd == "git" && args == ["status", "--short"] {
                return self.git_status.clone();
            }
            None
        }
    }

    #[test]
    fn test_git_provider_with_fake_executor() {
        let mut ctx = HarnessContext::default();
        let fake = FakeCommandExecutor {
            git_branch: Some("main".to_string()),
            git_status: Some(" M src/main.rs\n M src/engine.rs".to_string()),
        };

        let provider = GitProvider::new(fake);
        provider.provide_with_executor(&mut ctx);

        assert!(ctx.git_context.is_some());
        let git_str = ctx.git_context.unwrap();
        assert!(git_str.contains("Branch: main"));
        assert!(git_str.contains("Modified Files (2)"));
    }
}
