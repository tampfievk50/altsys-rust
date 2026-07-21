use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::rest::payload::AgentExecutionPayloads::ExecuteAgentRequest;
use crate::rest::payload::AgentPayloads::{CreateAgentRequest, UpdateAgentRequest};
use crate::rest::payload::AutomationRulePayloads::{CreateAutomationRuleRequest, UpdateAutomationRuleRequest};
use crate::rest::payload::CredentialPayloads::{CreateCredentialRequest, UpdateCredentialRequest};
use crate::rest::payload::EventPayloads::IngestEventRequest;
use crate::rest::payload::JiraWebhookPayloads::{JiraWebhookIssue, JiraWebhookIssueFields, JiraWebhookNamedField, JiraWebhookPayload};
use crate::rest::payload::KnowledgeItemPayloads::{CreateKnowledgeItemRequest, UpdateKnowledgeItemRequest};
use crate::rest::payload::ModelPayloads::{CreateModelRequest, UpdateModelRequest};
use crate::rest::payload::PluginPayloads::{CreatePluginRequest, UpdatePluginRequest};
use crate::rest::payload::ProjectPayloads::{CreateProjectRequest, UpdateProjectRequest};
use crate::rest::payload::PromptPayloads::{CreatePromptRequest, UpdatePromptRequest};
use crate::rest::payload::SdlcRunPayloads::StartSdlcRunRequest;
use crate::rest::payload::SemanticSearchPayloads::SemanticSearchRequest;
use crate::rest::payload::SkillPayloads::{CreateSkillRequest, UpdateSkillRequest};
use crate::rest::payload::TaskOverridePayloads::UpdateTaskSummaryRequest;
use crate::rest::payload::ToolPayloads::{CreateToolRequest, ExecuteToolRequest, UpdateToolRequest};
use crate::rest::payload::WorkflowDefinitionPayloads::{CreateWorkflowDefinitionRequest, UpdateWorkflowDefinitionRequest};
use crate::rest::payload::WorkflowExecutionPayloads::{ApprovalDecisionRequest, StartWorkflowExecutionRequest};
use crate::rest::payload::WorkflowTemplatePayloads::{CreateWorkflowTemplateRequest, InstantiateTemplateRequest, UpdateWorkflowTemplateRequest};
use crate::rest::response::ApiResponse::ApiResponse;

use sdlc_domain::dto::AgentExecutionResponse::AgentExecutionResponse;
use sdlc_domain::dto::AgentResponse::AgentResponse;
use sdlc_domain::dto::AgentType::AgentType;
use sdlc_domain::dto::AutomationRuleResponse::AutomationRuleResponse;
use sdlc_domain::dto::CredentialResponse::CredentialResponse;
use sdlc_domain::dto::CredentialSecretResponse::CredentialSecretResponse;
use sdlc_domain::dto::EventResponse::EventResponse;
use sdlc_domain::dto::IngestEventResponse::IngestEventResponse;
use sdlc_domain::dto::InstantiateTemplateResponse::InstantiateTemplateResponse;
use sdlc_domain::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use sdlc_domain::dto::KnowledgeSearchResult::KnowledgeSearchResult;
use sdlc_domain::dto::ModelResponse::ModelResponse;
use sdlc_domain::dto::PluginResponse::PluginResponse;
use sdlc_domain::dto::ProjectResponse::ProjectResponse;
use sdlc_domain::dto::PromptResponse::PromptResponse;
use sdlc_domain::dto::RuleFiringResponse::RuleFiringResponse;
use sdlc_domain::dto::SdlcRunResponse::SdlcRunResponse;
use sdlc_domain::dto::SdlcStepExecutionResponse::SdlcStepExecutionResponse;
use sdlc_domain::dto::SkillResponse::SkillResponse;
use sdlc_domain::dto::TaskOverrideResponse::TaskOverrideResponse;
use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use sdlc_domain::dto::ToolResponse::ToolResponse;
use sdlc_domain::dto::WorkflowDefinitionResponse::WorkflowDefinitionResponse;
use sdlc_domain::dto::WorkflowExecutionResponse::WorkflowExecutionResponse;
use sdlc_domain::dto::WorkflowNodeExecutionResponse::WorkflowNodeExecutionResponse;
use sdlc_domain::dto::WorkflowTemplateResponse::WorkflowTemplateResponse;

use crate::rest::controller::{
    AgentController, AgentExecutionController, AutomationRuleController, CredentialController,
    EventController, JiraWebhookController, KnowledgeItemController, ModelController, PluginController,
    ProjectController, PromptController, SdlcRunController, SemanticSearchController,
    SkillController, TaskController, ToolController, ToolExecutionController, WorkflowDefinitionController,
    WorkflowExecutionController, WorkflowTemplateController,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        ProjectController::create_project,
        ProjectController::get_projects_by_tenant,
        ProjectController::get_project,
        ProjectController::update_project,
        ProjectController::delete_project,

        CredentialController::create_credential,
        CredentialController::get_credentials_by_tenant,
        CredentialController::get_credential,
        CredentialController::reveal_credential_secret,
        CredentialController::update_credential,
        CredentialController::delete_credential,

        ModelController::create_model,
        ModelController::get_models_by_tenant,
        ModelController::get_model,
        ModelController::update_model,
        ModelController::delete_model,

        PromptController::create_prompt,
        PromptController::get_prompts_by_tenant,
        PromptController::get_prompt,
        PromptController::get_latest_prompt_by_key,
        PromptController::get_prompt_versions_by_key,
        PromptController::update_prompt,
        PromptController::delete_prompt,

        ToolController::create_tool,
        ToolController::get_tools_by_tenant,
        ToolController::get_tool,
        ToolController::update_tool,
        ToolController::delete_tool,
        ToolExecutionController::execute_tool,

        KnowledgeItemController::create_knowledge_item,
        KnowledgeItemController::get_knowledge_items_by_tenant,
        KnowledgeItemController::get_knowledge_item,
        KnowledgeItemController::get_latest_knowledge_item_by_key,
        KnowledgeItemController::get_knowledge_item_versions_by_key,
        KnowledgeItemController::update_knowledge_item,
        KnowledgeItemController::delete_knowledge_item,
        SemanticSearchController::search_knowledge,

        WorkflowDefinitionController::create_workflow_definition,
        WorkflowDefinitionController::get_workflow_definitions_by_tenant,
        WorkflowDefinitionController::get_workflow_definition,
        WorkflowDefinitionController::get_latest_workflow_definition_by_key,
        WorkflowDefinitionController::get_workflow_definition_versions_by_key,
        WorkflowDefinitionController::update_workflow_definition,
        WorkflowDefinitionController::delete_workflow_definition,
        WorkflowExecutionController::start_execution,
        WorkflowExecutionController::get_execution,
        WorkflowExecutionController::get_executions_by_tenant,
        WorkflowExecutionController::get_node_executions,
        WorkflowExecutionController::decide_approval,

        AgentController::create_agent,
        AgentController::get_agents_by_tenant,
        AgentController::get_agent,
        AgentController::update_agent,
        AgentController::delete_agent,
        AgentExecutionController::execute_agent,
        AgentExecutionController::get_execution,
        AgentExecutionController::get_executions_by_agent,

        SkillController::create_skill,
        SkillController::get_skills_by_tenant,
        SkillController::get_skill,
        SkillController::update_skill,
        SkillController::delete_skill,

        SdlcRunController::start_run,
        SdlcRunController::get_run,
        SdlcRunController::get_runs_by_tenant,
        SdlcRunController::get_step_executions,

        PluginController::create_plugin,
        PluginController::get_plugins_by_tenant,
        PluginController::get_plugin,
        PluginController::update_plugin,
        PluginController::delete_plugin,
        AutomationRuleController::create_rule,
        AutomationRuleController::get_rules_by_tenant,
        AutomationRuleController::get_rule,
        AutomationRuleController::update_rule,
        AutomationRuleController::delete_rule,
        WorkflowTemplateController::create_template,
        WorkflowTemplateController::get_templates_by_tenant,
        WorkflowTemplateController::get_template,
        WorkflowTemplateController::get_latest_template_by_key,
        WorkflowTemplateController::get_template_versions_by_key,
        WorkflowTemplateController::update_template,
        WorkflowTemplateController::delete_template,
        WorkflowTemplateController::instantiate_template,
        EventController::ingest_event,
        EventController::get_events_by_tenant,
        EventController::get_firings_by_event,
        JiraWebhookController::receive_jira_webhook,
        TaskController::get_task_overrides,
        TaskController::update_task_summary,
    ),
    components(
        schemas(
            CreateProjectRequest, UpdateProjectRequest, ApiResponse<ProjectResponse>, ApiResponse<Vec<ProjectResponse>>, ProjectResponse,
            CreateCredentialRequest, UpdateCredentialRequest, ApiResponse<CredentialResponse>, ApiResponse<Vec<CredentialResponse>>, ApiResponse<CredentialSecretResponse>, CredentialResponse, CredentialSecretResponse,
            CreateModelRequest, UpdateModelRequest, ApiResponse<ModelResponse>, ApiResponse<Vec<ModelResponse>>, ModelResponse,
            CreatePromptRequest, UpdatePromptRequest, ApiResponse<PromptResponse>, ApiResponse<Vec<PromptResponse>>, PromptResponse,

            CreateToolRequest, UpdateToolRequest, ApiResponse<ToolResponse>, ApiResponse<Vec<ToolResponse>>, ToolResponse,
            ExecuteToolRequest, ApiResponse<ToolExecutionResult>, ToolExecutionResult,

            CreateKnowledgeItemRequest, UpdateKnowledgeItemRequest, ApiResponse<KnowledgeItemResponse>, ApiResponse<Vec<KnowledgeItemResponse>>, KnowledgeItemResponse,
            SemanticSearchRequest, ApiResponse<Vec<KnowledgeSearchResult>>, KnowledgeSearchResult,

            CreateWorkflowDefinitionRequest, UpdateWorkflowDefinitionRequest, ApiResponse<WorkflowDefinitionResponse>, ApiResponse<Vec<WorkflowDefinitionResponse>>, WorkflowDefinitionResponse,
            StartWorkflowExecutionRequest, ApprovalDecisionRequest, ApiResponse<WorkflowExecutionResponse>, ApiResponse<Vec<WorkflowExecutionResponse>>, WorkflowExecutionResponse,
            ApiResponse<Vec<WorkflowNodeExecutionResponse>>, WorkflowNodeExecutionResponse,

            CreateAgentRequest, UpdateAgentRequest, AgentType, ApiResponse<AgentResponse>, ApiResponse<Vec<AgentResponse>>, AgentResponse,
            ExecuteAgentRequest, ApiResponse<AgentExecutionResponse>, ApiResponse<Vec<AgentExecutionResponse>>, AgentExecutionResponse,

            CreateSkillRequest, UpdateSkillRequest, ApiResponse<SkillResponse>, ApiResponse<Vec<SkillResponse>>, SkillResponse,

            StartSdlcRunRequest, ApiResponse<SdlcRunResponse>, ApiResponse<Vec<SdlcRunResponse>>, SdlcRunResponse,
            ApiResponse<Vec<SdlcStepExecutionResponse>>, SdlcStepExecutionResponse,

            CreatePluginRequest, UpdatePluginRequest, PluginResponse,
            CreateAutomationRuleRequest, UpdateAutomationRuleRequest, AutomationRuleResponse,
            CreateWorkflowTemplateRequest, UpdateWorkflowTemplateRequest, InstantiateTemplateRequest, WorkflowTemplateResponse, InstantiateTemplateResponse,
            IngestEventRequest, EventResponse, RuleFiringResponse, IngestEventResponse,
            JiraWebhookPayload, JiraWebhookIssue, JiraWebhookIssueFields, JiraWebhookNamedField,
            UpdateTaskSummaryRequest, ApiResponse<TaskOverrideResponse>, ApiResponse<Vec<TaskOverrideResponse>>, TaskOverrideResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Projects", description = "Project management APIs"),
        (name = "Credentials", description = "Credential management APIs"),
        (name = "Models", description = "Model registry APIs"),
        (name = "Prompts", description = "Prompt registry APIs"),
        (name = "Tools", description = "Tool registry APIs"),
        (name = "Tool Execution", description = "Tool SDK execution APIs (Git, GitHub, Jira, Filesystem, Cargo, Maven, Gradle)"),
        (name = "Knowledge", description = "Knowledge item registry and versioning APIs"),
        (name = "Knowledge Search", description = "Semantic search over embedded knowledge items"),
        (name = "Workflow Definitions", description = "Workflow graph registry and versioning APIs"),
        (name = "Workflow Executions", description = "Graph execution, checkpoints, retries, parallel execution, and human approvals"),
        (name = "Agents", description = "Agent registry APIs (Planner, Architect, Developer, Reviewer, Tester, Documentation)"),
        (name = "Agent Execution", description = "Agent Runtime execution APIs (Rig-backed LLM completion)"),
        (name = "Skills", description = "Reusable skill registry: instructions attachable to multiple agents and folded into their system prompt at execution time"),
        (name = "Autonomous SDLC", description = "Orchestrates Platform, Knowledge, Agents, and Tools into the ticket-to-pull-request pipeline"),
        (name = "Plugins", description = "Plugin SDK: register webhook-based extensions invoked by automation rules"),
        (name = "Automation Rules", description = "Event-driven triggers dispatching to SDLC, Workflow, Tools, Plugins, or a Classifier agent that picks the workflow"),
        (name = "Workflow Templates", description = "Versioned, parameterized workflow graph templates"),
        (name = "Events", description = "Event ingestion and the rule-firing audit trail"),
        (name = "Jira Webhook", description = "Inbound Jira Automation webhook per project; normalizes issues into events"),
        (name = "Tasks", description = "Per-project Jira ticket summary overrides: writes through to the real Jira ticket, then records the same value locally"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
