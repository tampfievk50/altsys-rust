use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantCommand {
    pub name: String,
    pub slug: String,
}
