use crate::tool_executor::BuildToolExecutor::BuildToolExecutor;

/// `mvn` build-tool executor (`tool_type = "maven"`). Supported actions:
/// `build` (compile), `test`, `clean`, `package`, `run` (with a free-form `args` parameter).
pub fn maven_executor() -> BuildToolExecutor {
    BuildToolExecutor::new("maven", "mvn")
}
