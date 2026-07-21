use std::collections::HashMap;

/// Everything a `ToolExecutorPort` needs to carry out one invocation: the
/// registered tool's own config plus the caller-supplied action/parameters.
#[derive(Debug, Clone)]
pub struct ToolExecutionContext {
    pub config: Option<String>,
    pub action: String,
    pub parameters: HashMap<String, String>,
    pub working_directory: Option<String>,
}
