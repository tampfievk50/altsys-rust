use crate::tool_executor::BuildToolExecutor::BuildToolExecutor;

/// `cargo` build-tool executor (`tool_type = "cargo"`). Supported actions:
/// `build`, `test`, `clean`, `run` (with a free-form `args` parameter).
pub fn cargo_executor() -> BuildToolExecutor {
    BuildToolExecutor::new("cargo", "cargo")
}
