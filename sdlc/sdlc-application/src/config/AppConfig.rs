use std::sync::Arc;
use std::time::Duration;
use sea_orm::DatabaseConnection;

use crate::state::AppState::AppState;

// ---- Platform ----
use sdlc_dataaccess::project::adapter::ProjectRepositoryImpl::ProjectRepositoryImpl;
use sdlc_dataaccess::project::repository::ProjectSeaOrmRepository::ProjectSeaOrmRepository;
use sdlc_dataaccess::task_override::adapter::TaskOverrideRepositoryImpl::TaskOverrideRepositoryImpl;
use sdlc_dataaccess::task_override::repository::TaskOverrideSeaOrmRepository::TaskOverrideSeaOrmRepository;
use sdlc_dataaccess::credential::adapter::CredentialRepositoryImpl::CredentialRepositoryImpl;
use sdlc_dataaccess::credential::repository::CredentialSeaOrmRepository::CredentialSeaOrmRepository;
use sdlc_dataaccess::model::adapter::ModelRepositoryImpl::ModelRepositoryImpl;
use sdlc_dataaccess::model::repository::ModelSeaOrmRepository::ModelSeaOrmRepository;
use sdlc_dataaccess::prompt::adapter::PromptRepositoryImpl::PromptRepositoryImpl;
use sdlc_dataaccess::prompt::repository::PromptSeaOrmRepository::PromptSeaOrmRepository;

use sdlc_domain::port::input::ProjectPort::ProjectPort;
use sdlc_domain::port::input::TaskOverridePort::TaskOverridePort;
use sdlc_domain::port::input::CredentialPort::CredentialPort;
use sdlc_domain::port::input::ModelPort::ModelPort;
use sdlc_domain::port::input::PromptPort::PromptPort;

use sdlc_domain::service::ProjectService::ProjectService;
use sdlc_domain::service::TaskOverrideService::TaskOverrideService;
use sdlc_domain::service::CredentialService::CredentialService;
use sdlc_domain::service::ModelService::ModelService;
use sdlc_domain::service::PromptService::PromptService;

// ---- Tools ----
use sdlc_dataaccess::tool_executor::CargoExecutor::cargo_executor;
use sdlc_dataaccess::tool_executor::FilesystemExecutor::FilesystemExecutor;
use sdlc_dataaccess::tool_executor::GitExecutor::GitExecutor;
use sdlc_dataaccess::tool_executor::GitHubExecutor::GitHubExecutor;
use sdlc_dataaccess::tool_executor::GradleExecutor::gradle_executor;
use sdlc_dataaccess::tool_executor::JiraExecutor::JiraExecutor;
use sdlc_dataaccess::tool_executor::MavenExecutor::maven_executor;
use sdlc_dataaccess::tool::adapter::ToolRepositoryImpl::ToolRepositoryImpl;
use sdlc_dataaccess::tool::repository::ToolSeaOrmRepository::ToolSeaOrmRepository;

use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::input::ToolPort::ToolPort;
use sdlc_domain::port::output::ToolExecutorPort::ToolExecutorPort;

use sdlc_domain::service::ToolExecutionService::ToolExecutionService;
use sdlc_domain::service::ToolService::ToolService;

// ---- Knowledge ----
use sdlc_dataaccess::embedding::LocalHashEmbeddingProvider::LocalHashEmbeddingProvider;
use sdlc_dataaccess::knowledge_item::adapter::KnowledgeItemRepositoryImpl::KnowledgeItemRepositoryImpl;
use sdlc_dataaccess::knowledge_item::repository::KnowledgeItemSeaOrmRepository::KnowledgeItemSeaOrmRepository;

use sdlc_domain::port::input::KnowledgePort::KnowledgePort;
use sdlc_domain::port::input::SemanticSearchPort::SemanticSearchPort;
use sdlc_domain::port::output::EmbeddingProviderPort::EmbeddingProviderPort;

use sdlc_domain::service::KnowledgeSearchService::KnowledgeSearchService;
use sdlc_domain::service::KnowledgeService::KnowledgeService;

// ---- Workflow ----
use sdlc_dataaccess::node_executor::AgentNodeExecutor::AgentNodeExecutor;
use sdlc_dataaccess::node_executor::NoOpNodeExecutor::NoOpNodeExecutor;
use sdlc_dataaccess::node_executor::ToolNodeExecutor::ToolNodeExecutor;
use sdlc_dataaccess::workflow_definition::adapter::WorkflowDefinitionRepositoryImpl::WorkflowDefinitionRepositoryImpl;
use sdlc_dataaccess::workflow_definition::repository::WorkflowDefinitionSeaOrmRepository::WorkflowDefinitionSeaOrmRepository;
use sdlc_dataaccess::workflow_execution::adapter::WorkflowExecutionRepositoryImpl::WorkflowExecutionRepositoryImpl;
use sdlc_dataaccess::workflow_execution::repository::WorkflowExecutionSeaOrmRepository::WorkflowExecutionSeaOrmRepository;
use sdlc_dataaccess::workflow_node_execution::adapter::WorkflowNodeExecutionRepositoryImpl::WorkflowNodeExecutionRepositoryImpl;
use sdlc_dataaccess::workflow_node_execution::repository::WorkflowNodeExecutionSeaOrmRepository::WorkflowNodeExecutionSeaOrmRepository;

use sdlc_domain::port::input::WorkflowDefinitionPort::WorkflowDefinitionPort;
use sdlc_domain::port::input::WorkflowExecutionPort::WorkflowExecutionPort;
use sdlc_domain::port::output::NodeExecutorPort::NodeExecutorPort;

use sdlc_domain::service::WorkflowDefinitionService::WorkflowDefinitionService;
use sdlc_domain::service::WorkflowEngineService::WorkflowEngineService;

// ---- Agents ----
use sdlc_dataaccess::agent::adapter::AgentRepositoryImpl::AgentRepositoryImpl;
use sdlc_dataaccess::agent::repository::AgentSeaOrmRepository::AgentSeaOrmRepository;
use sdlc_dataaccess::agent_execution::adapter::AgentExecutionRepositoryImpl::AgentExecutionRepositoryImpl;
use sdlc_dataaccess::agent_execution::repository::AgentExecutionSeaOrmRepository::AgentExecutionSeaOrmRepository;
use sdlc_dataaccess::llm::AnthropicLlmClient::AnthropicLlmClient;
use sdlc_dataaccess::llm::EchoLlmClient::EchoLlmClient;
use sdlc_dataaccess::llm::OpenAiLlmClient::OpenAiLlmClient;
use sdlc_dataaccess::skill::adapter::SkillRepositoryImpl::SkillRepositoryImpl;
use sdlc_dataaccess::skill::repository::SkillSeaOrmRepository::SkillSeaOrmRepository;

use sdlc_domain::port::input::AgentExecutionPort::AgentExecutionPort;
use sdlc_domain::port::input::AgentPort::AgentPort;
use sdlc_domain::port::input::SkillPort::SkillPort;
use sdlc_domain::port::output::LlmClientPort::LlmClientPort;

use sdlc_domain::service::AgentExecutionService::AgentExecutionService;
use sdlc_domain::service::AgentService::AgentService;
use sdlc_domain::service::SkillService::SkillService;

// ---- SDLC (was HTTP -> platform/tools/knowledge/agents, now in-process) ----
use sdlc_dataaccess::client::InProcessPlatformClient::InProcessPlatformClient;
use sdlc_dataaccess::client::InProcessSdlcToolsClient::InProcessSdlcToolsClient;
use sdlc_dataaccess::client::InProcessKnowledgeClient::InProcessKnowledgeClient;
use sdlc_dataaccess::client::InProcessAgentsClient::InProcessAgentsClient;
use sdlc_dataaccess::sdlc_checkpoint::adapter::SdlcCheckpointRepositoryImpl::SdlcCheckpointRepositoryImpl;
use sdlc_dataaccess::sdlc_run::adapter::SdlcRunRepositoryImpl::SdlcRunRepositoryImpl;
use sdlc_dataaccess::sdlc_run::repository::SdlcRunSeaOrmRepository::SdlcRunSeaOrmRepository;
use sdlc_dataaccess::sdlc_step_execution::adapter::SdlcStepExecutionRepositoryImpl::SdlcStepExecutionRepositoryImpl;
use sdlc_dataaccess::sdlc_step_execution::repository::SdlcStepExecutionSeaOrmRepository::SdlcStepExecutionSeaOrmRepository;

use sdlc_domain::port::input::SdlcRunPort::SdlcRunPort;
use sdlc_domain::service::SdlcOrchestratorService::SdlcOrchestratorService;

// ---- Automation (was HTTP -> workflow/sdlc/tools, now in-process) ----
use sdlc_dataaccess::automation_rule::adapter::AutomationRuleRepositoryImpl::AutomationRuleRepositoryImpl;
use sdlc_dataaccess::automation_rule::repository::AutomationRuleSeaOrmRepository::AutomationRuleSeaOrmRepository;
use sdlc_dataaccess::client::InProcessWorkflowClient::InProcessWorkflowClient;
use sdlc_dataaccess::client::InProcessSdlcClient::InProcessSdlcClient;
use sdlc_dataaccess::client::InProcessAutomationToolsClient::InProcessAutomationToolsClient;
use sdlc_dataaccess::client::PluginHttpDispatcher::PluginHttpDispatcher;
use sdlc_dataaccess::event::adapter::EventRepositoryImpl::EventRepositoryImpl;
use sdlc_dataaccess::event::repository::EventSeaOrmRepository::EventSeaOrmRepository;
use sdlc_dataaccess::plugin::adapter::PluginRepositoryImpl::PluginRepositoryImpl;
use sdlc_dataaccess::plugin::repository::PluginSeaOrmRepository::PluginSeaOrmRepository;
use sdlc_dataaccess::rule_firing::adapter::RuleFiringRepositoryImpl::RuleFiringRepositoryImpl;
use sdlc_dataaccess::rule_firing::repository::RuleFiringSeaOrmRepository::RuleFiringSeaOrmRepository;
use sdlc_dataaccess::workflow_template::adapter::WorkflowTemplateRepositoryImpl::WorkflowTemplateRepositoryImpl;
use sdlc_dataaccess::workflow_template::repository::WorkflowTemplateSeaOrmRepository::WorkflowTemplateSeaOrmRepository;

use sdlc_domain::port::input::AutomationRulePort::AutomationRulePort;
use sdlc_domain::port::input::EventPort::EventPort;
use sdlc_domain::port::input::PluginPort::PluginPort;
use sdlc_domain::port::input::WorkflowTemplatePort::WorkflowTemplatePort;

use sdlc_domain::service::AutomationRuleService::AutomationRuleService;
use sdlc_domain::service::EventAutomationService::EventAutomationService;
use sdlc_domain::service::JiraPollingScheduler::JiraPollingScheduler;
use sdlc_domain::service::PluginService::PluginService;
use sdlc_domain::service::WorkflowTemplateService::WorkflowTemplateService;

pub async fn create_app_state(db: DatabaseConnection) -> Arc<AppState> {
    // ===== Platform =====
    let project_repo = Arc::new(ProjectRepositoryImpl::new(ProjectSeaOrmRepository::new(db.clone())));

    let task_override_repo = Arc::new(TaskOverrideRepositoryImpl::new(TaskOverrideSeaOrmRepository::new(db.clone())));
    let task_override_service = Arc::new(TaskOverrideService::new(task_override_repo)) as Arc<dyn TaskOverridePort>;

    let credential_repo = Arc::new(CredentialRepositoryImpl::new(CredentialSeaOrmRepository::new(db.clone())));
    let credential_service = Arc::new(CredentialService::new(credential_repo.clone())) as Arc<dyn CredentialPort>;

    let model_repo = Arc::new(ModelRepositoryImpl::new(ModelSeaOrmRepository::new(db.clone())));
    let model_service = Arc::new(ModelService::new(model_repo.clone())) as Arc<dyn ModelPort>;

    let prompt_repo = Arc::new(PromptRepositoryImpl::new(PromptSeaOrmRepository::new(db.clone())));
    let prompt_service = Arc::new(PromptService::new(prompt_repo.clone())) as Arc<dyn PromptPort>;

    // ===== Tools =====
    let tool_repo = Arc::new(ToolRepositoryImpl::new(ToolSeaOrmRepository::new(db.clone())));
    let tool_service = Arc::new(ToolService::new(tool_repo.clone())) as Arc<dyn ToolPort>;

    let project_service = Arc::new(ProjectService::new(project_repo.clone(), tool_repo.clone())) as Arc<dyn ProjectPort>;

    let tool_executors: Vec<Arc<dyn ToolExecutorPort>> = vec![
        Arc::new(GitExecutor),
        Arc::new(GitHubExecutor),
        Arc::new(JiraExecutor),
        Arc::new(FilesystemExecutor),
        Arc::new(cargo_executor()),
        Arc::new(maven_executor()),
        Arc::new(gradle_executor()),
    ];
    let tool_execution_service = Arc::new(ToolExecutionService::new(tool_repo.clone(), tool_executors)) as Arc<dyn ToolExecutionPort>;

    // ===== Knowledge =====
    let knowledge_repo = Arc::new(KnowledgeItemRepositoryImpl::new(KnowledgeItemSeaOrmRepository::new(db.clone())));
    let embedding_provider = Arc::new(LocalHashEmbeddingProvider::default()) as Arc<dyn EmbeddingProviderPort>;

    let knowledge_service = Arc::new(KnowledgeService::new(knowledge_repo.clone(), embedding_provider.clone())) as Arc<dyn KnowledgePort>;
    let search_service = Arc::new(KnowledgeSearchService::new(knowledge_repo.clone(), embedding_provider.clone())) as Arc<dyn SemanticSearchPort>;

    // ===== Agents =====
    // Built before Workflow/SDLC/Automation below since all three need to hand a
    // Tools/Agents client to something they construct (node executors, the SDLC
    // orchestrator, the automation dispatcher) — one shared client per adapter,
    // reused via `.clone()` rather than built again at each call site.
    let skill_repo = Arc::new(SkillRepositoryImpl::new(SkillSeaOrmRepository::new(db.clone())));
    let skill_service = Arc::new(SkillService::new(skill_repo.clone())) as Arc<dyn SkillPort>;

    let agent_repo = Arc::new(AgentRepositoryImpl::new(AgentSeaOrmRepository::new(db.clone())));
    let agent_service = Arc::new(AgentService::new(agent_repo.clone(), skill_repo.clone())) as Arc<dyn AgentPort>;

    let agent_execution_repo = Arc::new(AgentExecutionRepositoryImpl::new(AgentExecutionSeaOrmRepository::new(db.clone())));
    let llm_clients: Vec<Arc<dyn LlmClientPort>> = vec![
        Arc::new(EchoLlmClient),
        Arc::new(OpenAiLlmClient),
        Arc::new(AnthropicLlmClient),
    ];
    let agent_execution_service = Arc::new(AgentExecutionService::new(agent_repo, agent_execution_repo, skill_repo, llm_clients)) as Arc<dyn AgentExecutionPort>;

    let agents_client = Arc::new(InProcessAgentsClient::new(agent_execution_service.clone()));
    let automation_tools_client = Arc::new(InProcessAutomationToolsClient::new(tool_execution_service.clone()));

    // ===== Workflow =====
    let definition_repo = Arc::new(WorkflowDefinitionRepositoryImpl::new(WorkflowDefinitionSeaOrmRepository::new(db.clone())));
    let workflow_execution_repo = Arc::new(WorkflowExecutionRepositoryImpl::new(WorkflowExecutionSeaOrmRepository::new(db.clone())));
    let node_execution_repo = Arc::new(WorkflowNodeExecutionRepositoryImpl::new(WorkflowNodeExecutionSeaOrmRepository::new(db.clone())));

    let definition_service = Arc::new(WorkflowDefinitionService::new(definition_repo.clone())) as Arc<dyn WorkflowDefinitionPort>;

    let node_executors: Vec<Arc<dyn NodeExecutorPort>> = vec![
        Arc::new(NoOpNodeExecutor),
        Arc::new(AgentNodeExecutor::new(agents_client.clone())),
        Arc::new(ToolNodeExecutor::new(automation_tools_client.clone())),
    ];
    let execution_service = Arc::new(WorkflowEngineService::new(
        definition_repo,
        workflow_execution_repo,
        node_execution_repo,
        node_executors,
    )) as Arc<dyn WorkflowExecutionPort>;

    // ===== SDLC =====
    // What used to be self-minted-JWT HTTP calls to platform/tools/knowledge/agents
    // are now direct in-process calls to the services built above.
    let sdlc_run_repo = Arc::new(SdlcRunRepositoryImpl::new(SdlcRunSeaOrmRepository::new(db.clone())));
    let sdlc_step_repo = Arc::new(SdlcStepExecutionRepositoryImpl::new(SdlcStepExecutionSeaOrmRepository::new(db.clone())));
    let sdlc_checkpoint_repo = Arc::new(SdlcCheckpointRepositoryImpl::new(db.clone()));

    let platform_client = Arc::new(InProcessPlatformClient::new(project_service.clone(), tool_repo.clone()));
    let sdlc_tools_client = Arc::new(InProcessSdlcToolsClient::new(tool_execution_service.clone()));
    let knowledge_client = Arc::new(InProcessKnowledgeClient::new(search_service.clone()));

    let sdlc_run_service = Arc::new(SdlcOrchestratorService::new(
        sdlc_run_repo,
        sdlc_step_repo,
        sdlc_checkpoint_repo,
        platform_client,
        knowledge_client,
        agents_client.clone(),
        sdlc_tools_client,
        tool_repo.clone(),
        credential_service.clone(),
    )) as Arc<dyn SdlcRunPort>;

    // ===== Automation =====
    // What used to be self-minted-JWT HTTP calls to workflow/sdlc/tools are now
    // direct in-process calls; only the plugin webhook dispatch stays real HTTP
    // (it calls genuinely external third-party systems).
    let plugin_repo = Arc::new(PluginRepositoryImpl::new(PluginSeaOrmRepository::new(db.clone())));
    let rule_repo = Arc::new(AutomationRuleRepositoryImpl::new(AutomationRuleSeaOrmRepository::new(db.clone())));
    let template_repo = Arc::new(WorkflowTemplateRepositoryImpl::new(WorkflowTemplateSeaOrmRepository::new(db.clone())));
    let event_repo = Arc::new(EventRepositoryImpl::new(EventSeaOrmRepository::new(db.clone())));
    let firing_repo = Arc::new(RuleFiringRepositoryImpl::new(RuleFiringSeaOrmRepository::new(db.clone())));

    let workflow_client = Arc::new(InProcessWorkflowClient::new(execution_service.clone(), definition_service.clone()));
    let sdlc_client = Arc::new(InProcessSdlcClient::new(sdlc_run_service.clone()));
    let plugin_dispatch = Arc::new(PluginHttpDispatcher::new());

    let plugin_service = Arc::new(PluginService::new(plugin_repo.clone())) as Arc<dyn PluginPort>;
    let rule_service = Arc::new(AutomationRuleService::new(rule_repo.clone())) as Arc<dyn AutomationRulePort>;
    let template_service = Arc::new(WorkflowTemplateService::new(template_repo)) as Arc<dyn WorkflowTemplatePort>;
    let event_service = Arc::new(EventAutomationService::new(
        event_repo, firing_repo, rule_repo, plugin_repo,
        workflow_client, sdlc_client, automation_tools_client, plugin_dispatch,
        agents_client.clone(),
    )) as Arc<dyn EventPort>;

    // Phase 3 fallback ingestion: polls Jira for projects that have no webhook
    // configured. Runs alongside the HTTP server rather than as a separate
    // process, matching how everything else in this service is in-process.
    let jira_scheduler = Arc::new(JiraPollingScheduler::new(
        project_repo.clone(),
        tool_repo.clone(),
        credential_service.clone(),
        tool_execution_service.clone(),
        event_service.clone(),
    ));
    let jira_poll_interval_secs: u64 = std::env::var("JIRA_POLL_INTERVAL_SECONDS")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(300);
    tokio::spawn(jira_scheduler.run_forever(Duration::from_secs(jira_poll_interval_secs)));

    Arc::new(AppState {
        project_service,
        task_override_service,
        credential_service,
        model_service,
        prompt_service,
        tool_service,
        tool_execution_service,
        knowledge_service,
        search_service,
        definition_service,
        execution_service,
        agent_service,
        agent_execution_service,
        skill_service,
        sdlc_run_service,
        plugin_service,
        rule_service,
        template_service,
        event_service,
    })
}
