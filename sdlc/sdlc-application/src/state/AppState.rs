use std::sync::Arc;

use sdlc_domain::port::input::AgentExecutionPort::AgentExecutionPort;
use sdlc_domain::port::input::AgentPort::AgentPort;
use sdlc_domain::port::input::AutomationRulePort::AutomationRulePort;
use sdlc_domain::port::input::CredentialPort::CredentialPort;
use sdlc_domain::port::input::EventPort::EventPort;
use sdlc_domain::port::input::KnowledgePort::KnowledgePort;
use sdlc_domain::port::input::ModelPort::ModelPort;
use sdlc_domain::port::input::PluginPort::PluginPort;
use sdlc_domain::port::input::ProjectPort::ProjectPort;
use sdlc_domain::port::input::PromptPort::PromptPort;
use sdlc_domain::port::input::SdlcRunPort::SdlcRunPort;
use sdlc_domain::port::input::SemanticSearchPort::SemanticSearchPort;
use sdlc_domain::port::input::SkillPort::SkillPort;
use sdlc_domain::port::input::TaskOverridePort::TaskOverridePort;
use sdlc_domain::port::input::ToolExecutionPort::ToolExecutionPort;
use sdlc_domain::port::input::ToolPort::ToolPort;
use sdlc_domain::port::input::WorkflowDefinitionPort::WorkflowDefinitionPort;
use sdlc_domain::port::input::WorkflowExecutionPort::WorkflowExecutionPort;
use sdlc_domain::port::input::WorkflowTemplatePort::WorkflowTemplatePort;

pub struct AppState {
    // Platform
    pub project_service: Arc<dyn ProjectPort>,
    pub task_override_service: Arc<dyn TaskOverridePort>,
    pub credential_service: Arc<dyn CredentialPort>,
    pub model_service: Arc<dyn ModelPort>,
    pub prompt_service: Arc<dyn PromptPort>,
    // Tools
    pub tool_service: Arc<dyn ToolPort>,
    pub tool_execution_service: Arc<dyn ToolExecutionPort>,
    // Knowledge
    pub knowledge_service: Arc<dyn KnowledgePort>,
    pub search_service: Arc<dyn SemanticSearchPort>,
    // Workflow
    pub definition_service: Arc<dyn WorkflowDefinitionPort>,
    pub execution_service: Arc<dyn WorkflowExecutionPort>,
    // Agents
    pub agent_service: Arc<dyn AgentPort>,
    pub agent_execution_service: Arc<dyn AgentExecutionPort>,
    pub skill_service: Arc<dyn SkillPort>,
    // SDLC
    pub sdlc_run_service: Arc<dyn SdlcRunPort>,
    // Automation
    pub plugin_service: Arc<dyn PluginPort>,
    pub rule_service: Arc<dyn AutomationRulePort>,
    pub template_service: Arc<dyn WorkflowTemplatePort>,
    pub event_service: Arc<dyn EventPort>,
}
