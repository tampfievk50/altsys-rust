use std::sync::Arc;
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::state::AppState::AppState;
use crate::middleware::AuthMiddleware::require_auth;
use crate::rest::controller::{
    AgentController, AgentExecutionController, AutomationRuleController, CredentialController,
    EventController, JiraWebhookController, KnowledgeItemController, ModelController, PluginController,
    ProjectController, PromptController, SdlcRunController, SemanticSearchController,
    SkillController, TaskController, ToolController, ToolExecutionController, WorkflowDefinitionController,
    WorkflowExecutionController, WorkflowTemplateController,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let management_routes = Router::new()
        // ===== Platform =====
        .route("/projects", post(ProjectController::create_project))
        .route("/tenants/{tenant_id}/projects", get(ProjectController::get_projects_by_tenant))
        .route("/projects/{id}", get(ProjectController::get_project).put(ProjectController::update_project).delete(ProjectController::delete_project))
        .route("/projects/{project_id}/task-overrides", get(TaskController::get_task_overrides))
        .route("/projects/{project_id}/tickets/{ticket_key}", put(TaskController::update_task_summary))
        .route("/credentials", post(CredentialController::create_credential))
        .route("/tenants/{tenant_id}/credentials", get(CredentialController::get_credentials_by_tenant))
        .route("/credentials/{id}", get(CredentialController::get_credential).put(CredentialController::update_credential).delete(CredentialController::delete_credential))
        .route("/credentials/{id}/secret", get(CredentialController::reveal_credential_secret))
        .route("/models", post(ModelController::create_model))
        .route("/tenants/{tenant_id}/models", get(ModelController::get_models_by_tenant))
        .route("/models/{id}", get(ModelController::get_model).put(ModelController::update_model).delete(ModelController::delete_model))
        .route("/prompts", post(PromptController::create_prompt))
        .route("/tenants/{tenant_id}/prompts", get(PromptController::get_prompts_by_tenant))
        .route("/prompts/{id}", get(PromptController::get_prompt).put(PromptController::update_prompt).delete(PromptController::delete_prompt))
        .route("/tenants/{tenant_id}/prompts/{key}/latest", get(PromptController::get_latest_prompt_by_key))
        .route("/tenants/{tenant_id}/prompts/{key}/versions", get(PromptController::get_prompt_versions_by_key))

        // ===== Tools =====
        .route("/tools", post(ToolController::create_tool))
        .route("/tenants/{tenant_id}/tools", get(ToolController::get_tools_by_tenant))
        .route("/tools/{id}", get(ToolController::get_tool).put(ToolController::update_tool).delete(ToolController::delete_tool))
        .route("/tools/{id}/execute", post(ToolExecutionController::execute_tool))

        // ===== Knowledge =====
        .route("/knowledge-items", post(KnowledgeItemController::create_knowledge_item))
        .route("/tenants/{tenant_id}/knowledge-items", get(KnowledgeItemController::get_knowledge_items_by_tenant))
        .route("/knowledge-items/{id}", get(KnowledgeItemController::get_knowledge_item).put(KnowledgeItemController::update_knowledge_item).delete(KnowledgeItemController::delete_knowledge_item))
        .route("/tenants/{tenant_id}/knowledge-items/{key}/latest", get(KnowledgeItemController::get_latest_knowledge_item_by_key))
        .route("/tenants/{tenant_id}/knowledge-items/{key}/versions", get(KnowledgeItemController::get_knowledge_item_versions_by_key))
        .route("/tenants/{tenant_id}/knowledge-items/search", post(SemanticSearchController::search_knowledge))

        // ===== Workflow =====
        .route("/workflow-definitions", post(WorkflowDefinitionController::create_workflow_definition))
        .route("/tenants/{tenant_id}/workflow-definitions", get(WorkflowDefinitionController::get_workflow_definitions_by_tenant))
        .route("/workflow-definitions/{id}", get(WorkflowDefinitionController::get_workflow_definition).put(WorkflowDefinitionController::update_workflow_definition).delete(WorkflowDefinitionController::delete_workflow_definition))
        .route("/tenants/{tenant_id}/workflow-definitions/{key}/latest", get(WorkflowDefinitionController::get_latest_workflow_definition_by_key))
        .route("/tenants/{tenant_id}/workflow-definitions/{key}/versions", get(WorkflowDefinitionController::get_workflow_definition_versions_by_key))
        .route("/workflow-executions", post(WorkflowExecutionController::start_execution))
        .route("/workflow-executions/{id}", get(WorkflowExecutionController::get_execution))
        .route("/tenants/{tenant_id}/workflow-executions", get(WorkflowExecutionController::get_executions_by_tenant))
        .route("/workflow-executions/{id}/node-executions", get(WorkflowExecutionController::get_node_executions))
        .route("/workflow-executions/{id}/nodes/{node_id}/decide", post(WorkflowExecutionController::decide_approval))

        // ===== Agents =====
        .route("/agents", post(AgentController::create_agent))
        .route("/tenants/{tenant_id}/agents", get(AgentController::get_agents_by_tenant))
        .route("/agents/{id}", get(AgentController::get_agent).put(AgentController::update_agent).delete(AgentController::delete_agent))
        .route("/agents/{id}/execute", post(AgentExecutionController::execute_agent))
        .route("/agents/{id}/executions", get(AgentExecutionController::get_executions_by_agent))
        .route("/agent-executions/{id}", get(AgentExecutionController::get_execution))
        .route("/skills", post(SkillController::create_skill))
        .route("/tenants/{tenant_id}/skills", get(SkillController::get_skills_by_tenant))
        .route("/skills/{id}", get(SkillController::get_skill).put(SkillController::update_skill).delete(SkillController::delete_skill))

        // ===== SDLC =====
        .route("/sdlc-runs", post(SdlcRunController::start_run))
        .route("/sdlc-runs/{id}", get(SdlcRunController::get_run))
        .route("/tenants/{tenant_id}/sdlc-runs", get(SdlcRunController::get_runs_by_tenant))
        .route("/sdlc-runs/{id}/steps", get(SdlcRunController::get_step_executions))

        // ===== Automation =====
        .route("/plugins", post(PluginController::create_plugin))
        .route("/tenants/{tenant_id}/plugins", get(PluginController::get_plugins_by_tenant))
        .route("/plugins/{id}", get(PluginController::get_plugin).put(PluginController::update_plugin).delete(PluginController::delete_plugin))
        .route("/automation-rules", post(AutomationRuleController::create_rule))
        .route("/tenants/{tenant_id}/automation-rules", get(AutomationRuleController::get_rules_by_tenant))
        .route("/automation-rules/{id}", get(AutomationRuleController::get_rule).put(AutomationRuleController::update_rule).delete(AutomationRuleController::delete_rule))
        .route("/workflow-templates", post(WorkflowTemplateController::create_template))
        .route("/tenants/{tenant_id}/workflow-templates", get(WorkflowTemplateController::get_templates_by_tenant))
        .route("/tenants/{tenant_id}/workflow-templates/{key}/latest", get(WorkflowTemplateController::get_latest_template_by_key))
        .route("/tenants/{tenant_id}/workflow-templates/{key}/versions", get(WorkflowTemplateController::get_template_versions_by_key))
        .route("/workflow-templates/{id}", get(WorkflowTemplateController::get_template).put(WorkflowTemplateController::update_template).delete(WorkflowTemplateController::delete_template))
        .route("/workflow-templates/{id}/instantiate", post(WorkflowTemplateController::instantiate_template))
        .route("/tenants/{tenant_id}/events", post(EventController::ingest_event).get(EventController::get_events_by_tenant))
        .route("/events/{id}/firings", get(EventController::get_firings_by_event))

        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Deliberately outside `require_auth` — external systems like Jira can't mint
    // our JWTs. Guarded instead by the per-project secret embedded in the path.
    let webhook_routes = Router::new()
        .route("/webhooks/jira/{project_id}/{secret}", post(JiraWebhookController::receive_jira_webhook));

    Router::new()
        .nest("/api/v1", management_routes)
        .nest("/api/v1", webhook_routes)
        .with_state(state)
}
