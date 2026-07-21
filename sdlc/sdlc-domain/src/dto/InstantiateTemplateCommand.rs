use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiateTemplateCommand {
    #[serde(default)]
    pub parameters: HashMap<String, String>,
}
