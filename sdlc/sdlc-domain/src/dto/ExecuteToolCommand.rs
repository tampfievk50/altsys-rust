use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteToolCommand {
    pub action: String,
    #[serde(default)]
    pub parameters: HashMap<String, String>,
    pub working_directory: Option<String>,
}
