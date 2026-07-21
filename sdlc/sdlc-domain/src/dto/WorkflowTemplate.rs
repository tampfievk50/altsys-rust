use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WorkflowTemplate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub name: String,
    pub description: Option<String>,
    /// Raw JSON text of a `WorkflowGraph` (see the Workflow service) containing
    /// `{{parameter}}` placeholders, resolved by `instantiate`.
    pub definition_template: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewWorkflowTemplate {
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub name: String,
    pub description: Option<String>,
    pub definition_template: String,
}

impl WorkflowTemplate {
    pub fn new(fields: NewWorkflowTemplate) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            key: fields.key,
            version: fields.version,
            name: fields.name,
            description: fields.description,
            definition_template: fields.definition_template,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    pub fn latest_per_key(items: Vec<WorkflowTemplate>) -> Vec<WorkflowTemplate> {
        use std::collections::HashMap;
        let mut latest: HashMap<String, WorkflowTemplate> = HashMap::new();
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

    /// Replaces every `{{key}}` occurrence with its parameter value. Missing
    /// parameters leave the placeholder untouched so the caller can spot them.
    pub fn instantiate(&self, parameters: &std::collections::HashMap<String, String>) -> String {
        let mut resolved = self.definition_template.clone();
        for (key, value) in parameters {
            resolved = resolved.replace(&format!("{{{{{}}}}}", key), value);
        }
        resolved
    }
}
