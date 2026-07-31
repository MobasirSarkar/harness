use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCategory {
    DiscoveryMcp,
    StructuralSearch,
    MemorySystem,
    OutputCompressor,
    FastSearch,
}

impl fmt::Display for ToolCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolCategory::DiscoveryMcp => write!(f, "Discovery MCP"),
            ToolCategory::StructuralSearch => write!(f, "Structural AST"),
            ToolCategory::MemorySystem => write!(f, "Persistent Memory"),
            ToolCategory::OutputCompressor => write!(f, "Output Compression"),
            ToolCategory::FastSearch => write!(f, "Fast Search CLI"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessToolSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub binary_name: &'static str,
    pub category: ToolCategory,
    pub usage_guideline: &'static str,
}

pub fn get_required_harness_tools() -> Vec<HarnessToolSpec> {
    vec![
        HarnessToolSpec {
            id: "fff",
            name: "FFF MCP Server",
            binary_name: "fff-mcp",
            category: ToolCategory::DiscoveryMcp,
            usage_guideline: "Sub-ms typo-tolerant file path resolution",
        },
        HarnessToolSpec {
            id: "codebase-memory-mcp",
            name: "Codebase Memory MCP",
            binary_name: "codebase-memory-mcp",
            category: ToolCategory::DiscoveryMcp,
            usage_guideline: "Deep structural AST call-graph queries (search_graph, trace_path)",
        },
        HarnessToolSpec {
            id: "engram",
            name: "Engram Persistent Memory",
            binary_name: "engram",
            category: ToolCategory::MemorySystem,
            usage_guideline: "Cross-session memory persistence (mem_save, mem_context)",
        },
        HarnessToolSpec {
            id: "ast-grep",
            name: "ast-grep (sg)",
            binary_name: "ast-grep",
            category: ToolCategory::StructuralSearch,
            usage_guideline: "Language-aware AST pattern matching & structural rewrites",
        },
        HarnessToolSpec {
            id: "rtk",
            name: "Rust Token Killer (RTK)",
            binary_name: "rtk",
            category: ToolCategory::OutputCompressor,
            usage_guideline: "60-90% token compression for CLI command outputs",
        },
        HarnessToolSpec {
            id: "rg",
            name: "ripgrep",
            binary_name: "rg",
            category: ToolCategory::FastSearch,
            usage_guideline: "Fast text searching for literals & regex",
        },
        HarnessToolSpec {
            id: "fd",
            name: "fd-find",
            binary_name: "fd",
            category: ToolCategory::FastSearch,
            usage_guideline: "Fast file path finding",
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolState {
    Active { path: String },
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolAuditResult {
    pub spec: HarnessToolSpec,
    pub state: ToolState,
}
