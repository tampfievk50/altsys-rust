use serde::Deserialize;
use utoipa::ToSchema;

/// The subset of Jira's webhook body (`jira:issue_created` / `jira:issue_updated`)
/// this endpoint needs — Jira sends many more fields, all ignored by serde.
#[derive(Debug, Deserialize, ToSchema)]
pub struct JiraWebhookPayload {
    #[serde(default, rename = "webhookEvent")]
    pub webhook_event: String,
    pub issue: JiraWebhookIssue,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JiraWebhookIssue {
    pub key: String,
    pub fields: JiraWebhookIssueFields,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JiraWebhookIssueFields {
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub issuetype: Option<JiraWebhookNamedField>,
    #[serde(default)]
    pub priority: Option<JiraWebhookNamedField>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JiraWebhookNamedField {
    pub name: String,
}
