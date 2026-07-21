use crate::tool_executor::BuildToolExecutor::BuildToolExecutor;

/// `gradle` build-tool executor (`tool_type = "gradle"`). Supported actions:
/// `build`, `test`, `clean`, `run` (with a free-form `args` parameter).
pub fn gradle_executor() -> BuildToolExecutor {
    BuildToolExecutor::new("gradle", "gradle")
}
