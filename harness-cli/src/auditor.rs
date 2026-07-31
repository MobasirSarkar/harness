use std::env;
use std::path::{Path, PathBuf};
use crate::tools::{get_required_harness_tools, HarnessToolSpec, ToolAuditResult, ToolState};

pub trait PathResolver {
    fn find_executable(&self, binary_name: &str) -> Option<PathBuf>;
}

pub struct SystemPathResolver;

impl PathResolver for SystemPathResolver {
    fn find_executable(&self, binary_name: &str) -> Option<PathBuf> {
        let home = env::var("HOME").ok()?;
        let known_paths = [
            format!("{}/.local/bin/{}", home, binary_name),
            format!("{}/go/bin/{}", home, binary_name),
            format!("{}/.cargo/bin/{}", home, binary_name),
            format!("/usr/local/bin/{}", binary_name),
            format!("/usr/bin/{}", binary_name),
        ];

        for p in &known_paths {
            let path = Path::new(p);
            if path.exists() && is_executable(path) {
                return Some(path.to_path_buf());
            }
        }

        if let Ok(path_var) = env::var("PATH") {
            for dir in env::split_paths(&path_var) {
                let full_path = dir.join(binary_name);
                if full_path.exists() && is_executable(&full_path) {
                    return Some(full_path);
                }
            }
        }

        None
    }
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = path.metadata() {
        metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0)
    } else {
        false
    }
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

pub struct ToolAuditor<R: PathResolver> {
    resolver: R,
}

impl ToolAuditor<SystemPathResolver> {
    pub fn default_system() -> Self {
        Self {
            resolver: SystemPathResolver,
        }
    }
}

impl<R: PathResolver> ToolAuditor<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    pub fn audit_all(&self) -> Vec<ToolAuditResult> {
        let tools = get_required_harness_tools();
        tools.into_iter().map(|spec| self.audit_spec(spec)).collect()
    }

    pub fn audit_spec(&self, spec: HarnessToolSpec) -> ToolAuditResult {
        let state = match self.resolver.find_executable(spec.binary_name) {
            Some(path) => ToolState::Active {
                path: path.to_string_lossy().to_string(),
            },
            None => ToolState::Missing,
        };
        ToolAuditResult { spec, state }
    }

    pub fn generate_summary_text(&self) -> String {
        let results = self.audit_all();
        let mut out = String::new();
        out.push_str("=== HARNESS TOOL INVOCATION TELEMETRY & HEALTH AUDIT ===\n");
        for res in results {
            match res.state {
                ToolState::Active { path } => {
                    out.push_str(&format!(
                        "✓ {:<22} [{}] -> ACTIVE ({})\n",
                        res.spec.name, res.spec.category, path
                    ));
                }
                ToolState::Missing => {
                    out.push_str(&format!(
                        "✗ {:<22} [{}] -> MISSING (Not found in PATH or ~/.local/bin)\n",
                        res.spec.name, res.spec.category
                    ));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    pub struct FakePathResolver {
        pub binaries: HashMap<String, PathBuf>,
    }

    impl PathResolver for FakePathResolver {
        fn find_executable(&self, binary_name: &str) -> Option<PathBuf> {
            self.binaries.get(binary_name).cloned()
        }
    }

    #[test]
    fn test_auditor_with_fake_resolver() {
        let mut binaries = HashMap::new();
        binaries.insert("rg".to_string(), PathBuf::from("/usr/bin/rg"));
        binaries.insert("fff-mcp".to_string(), PathBuf::from("/home/user/.local/bin/fff-mcp"));

        let fake = FakePathResolver { binaries };
        let auditor = ToolAuditor::new(fake);

        let results = auditor.audit_all();
        assert_eq!(results.len(), 7);

        let rg_res = results.iter().find(|r| r.spec.id == "rg").unwrap();
        assert_eq!(rg_res.state, ToolState::Active { path: "/usr/bin/rg".to_string() });

        let ast_res = results.iter().find(|r| r.spec.id == "ast-grep").unwrap();
        assert_eq!(ast_res.state, ToolState::Missing);
    }
}
