use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WorkflowDefinition {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub name: String,
    pub description: Option<String>,
    /// Raw JSON text parsed into a `WorkflowGraph` by the engine; validated at creation time.
    pub definition: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewWorkflowDefinition {
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
}

impl WorkflowDefinition {
    pub fn new(fields: NewWorkflowDefinition) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            key: fields.key,
            version: fields.version,
            name: fields.name,
            description: fields.description,
            definition: fields.definition,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    pub fn latest_per_key(items: Vec<WorkflowDefinition>) -> Vec<WorkflowDefinition> {
        use std::collections::HashMap;
        let mut latest: HashMap<String, WorkflowDefinition> = HashMap::new();
        for item in items {
            latest
                .entry(item.key.clone())
                .and_modify(|existing| {
                    if item.version > existing.version {
                        *existing = item.clone();
                    }
                })
                .or_insert(item);
        }
        latest.into_values().collect()
    }
}
