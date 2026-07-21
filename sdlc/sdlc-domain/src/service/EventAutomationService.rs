use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::ActionSpec::ActionSpec;
use crate::dto::EventResponse::EventResponse;
use crate::dto::IngestEventCommand::IngestEventCommand;
use crate::dto::IngestEventResponse::IngestEventResponse;
use crate::dto::IngestedEvent::{IngestedEvent, NewIngestedEvent};
use crate::dto::RuleFiring::{NewRuleFiring, RuleFiring};
use crate::dto::RuleFiringResponse::RuleFiringResponse;
use crate::dto::RuleFiringStatus::RuleFiringStatus;
use crate::port::input::EventPort::EventPort;
use crate::port::output::AgentsClientPort::AgentsClientPort;
use crate::port::output::AutomationRuleRepositoryPort::AutomationRuleRepositoryPort;
use crate::port::output::EventRepositoryPort::EventRepositoryPort;
use crate::port::output::PluginDispatchPort::PluginDispatchPort;
use crate::port::output::PluginRepositoryPort::PluginRepositoryPort;
use crate::port::output::RuleFiringRepositoryPort::RuleFiringRepositoryPort;
use crate::port::output::SdlcClientPort::SdlcClientPort;
use crate::port::output::AutomationToolsClientPort::AutomationToolsClientPort;
use crate::port::output::WorkflowClientPort::WorkflowClientPort;
use crate::r#enum::DomainError::DomainError;

pub struct EventAutomationService {
    event_repository: Arc<dyn EventRepositoryPort>,
    firing_repository: Arc<dyn RuleFiringRepositoryPort>,
    rule_repository: Arc<dyn AutomationRuleRepositoryPort>,
    plugin_repository: Arc<dyn PluginRepositoryPort>,
    workflow_client: Arc<dyn WorkflowClientPort>,
    sdlc_client: Arc<dyn SdlcClientPort>,
    tools_client: Arc<dyn AutomationToolsClientPort>,
    plugin_dispatch: Arc<dyn PluginDispatchPort>,
    agents_client: Arc<dyn AgentsClientPort>,
}

impl EventAutomationService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_repository: Arc<dyn EventRepositoryPort>,
        firing_repository: Arc<dyn RuleFiringRepositoryPort>,
        rule_repository: Arc<dyn AutomationRuleRepositoryPort>,
        plugin_repository: Arc<dyn PluginRepositoryPort>,
        workflow_client: Arc<dyn WorkflowClientPort>,
        sdlc_client: Arc<dyn SdlcClientPort>,
        tools_client: Arc<dyn AutomationToolsClientPort>,
        plugin_dispatch: Arc<dyn PluginDispatchPort>,
        agents_client: Arc<dyn AgentsClientPort>,
    ) -> Self {
        Self {
            event_repository, firing_repository, rule_repository, plugin_repository,
            workflow_client, sdlc_client, tools_client, plugin_dispatch, agents_client,
        }
    }

    fn to_event_response(event: &IngestedEvent) -> EventResponse {
        EventResponse {
            id: event.id,
            tenant_id: event.tenant_id,
            event_type: event.event_type.clone(),
            payload: event.payload.to_string(),
            received_at: event.received_at,
        }
    }

    fn to_firing_response(firing: &RuleFiring) -> RuleFiringResponse {
        RuleFiringResponse {
            id: firing.id,
            event_id: firing.event_id,
            rule_id: firing.rule_id,
            matched: firing.matched,
            status: firing.status.to_string(),
            action_result: firing.action_result.as_ref().map(|v| v.to_string()),
            error: firing.error.clone(),
            created_at: firing.created_at,
        }
    }

    /// Boxed because `ClassifyAndDispatch` recurses into `fallback_action` — an
    /// unboxed `async fn` can't call itself (the future's size would be infinite).
    fn dispatch_action<'a>(&'a self, action: &'a ActionSpec, event: &'a IngestedEvent) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, DomainError>> + Send + 'a>> {
        Box::pin(async move {
            match action {
                ActionSpec::StartSdlcRun { parameters } => {
                    self.sdlc_client.start_run(Self::merge_event_into_sdlc_parameters(parameters, event)).await
                }
                ActionSpec::StartWorkflowExecution { workflow_definition_id, parameters } => {
                    self.workflow_client.start_execution(*workflow_definition_id, event.tenant_id, parameters.clone()).await
                }
                ActionSpec::ExecuteTool { tool_id, action, parameters } => {
                    self.tools_client.execute_tool(*tool_id, action.clone(), parameters.clone()).await
                }
                ActionSpec::InvokePlugin { plugin_id, parameters } => {
                    let plugin = self.plugin_repository.find_by_id(*plugin_id).await?
                        .ok_or_else(|| DomainError::NotFound(format!("Plugin not found: {}", plugin_id)))?;
                    self.plugin_dispatch.dispatch(&plugin, &event.event_type, &event.payload, parameters).await
                }
                ActionSpec::ClassifyAndDispatch { classifier_agent_id, fallback_action } => {
                    match self.classify(*classifier_agent_id, event).await {
                        Some(workflow_definition_id) => {
                            self.workflow_client.start_execution(workflow_definition_id, event.tenant_id, event.payload.clone()).await
                        }
                        None => self.dispatch_action(fallback_action, event).await,
                    }
                }
            }
        })
    }

    /// Runs the classifier agent against the event payload and resolves its
    /// `workflow_key` output to a concrete `WorkflowDefinition` id. Returns `None`
    /// on any failure (agent error, unparseable output, missing/empty key, or no
    /// matching definition) so the caller can fall back to `fallback_action`.
    async fn classify(&self, classifier_agent_id: Uuid, event: &IngestedEvent) -> Option<Uuid> {
        let prompt = format!(
            "Classify this incoming task and choose which workflow should process it. \
             Respond with strict JSON only, no prose: {{\"workflow_key\": \"<key>\", \"task_type\": \"bug|feature|chore|other\"}}.\n\n\
             Task payload:\n{}",
            event.payload
        );
        let output = self.agents_client.execute_agent(classifier_agent_id, event.tenant_id, prompt).await
            .inspect_err(|e| error!(error = %e, "Classifier agent execution failed"))
            .ok()?;
        let json_slice = Self::extract_json_object(&output)?;
        let parsed: serde_json::Value = serde_json::from_str(json_slice).ok()?;
        let workflow_key = parsed.get("workflow_key")?.as_str()?;
        if workflow_key.trim().is_empty() {
            return None;
        }
        self.workflow_client.find_definition_id_by_key(event.tenant_id, workflow_key).await
            .inspect_err(|e| error!(error = %e, workflow_key, "No workflow definition matched classifier output"))
            .ok()
    }

    /// LLM output sometimes wraps the requested JSON in prose or a code fence;
    /// take the outermost `{...}` span rather than requiring an exact match.
    fn extract_json_object(text: &str) -> Option<&str> {
        let start = text.find('{')?;
        let end = text.rfind('}')?;
        if end < start {
            return None;
        }
        Some(&text[start..=end])
    }

    /// A rule's `StartSdlcRun` parameters are static (agent/tool ids that don't
    /// change per ticket); the fields that legitimately vary per ticket —
    /// `tenant_id`, `project_id`, `ticket_key`, `ticket_summary` — are overlaid
    /// from the triggering event's own tenant/payload, taking precedence over
    /// anything the rule author put in `parameters` for those specific keys.
    fn merge_event_into_sdlc_parameters(parameters: &serde_json::Value, event: &IngestedEvent) -> serde_json::Value {
        let mut merged = parameters.clone();
        let Some(map) = merged.as_object_mut() else { return merged };
        map.insert("tenant_id".to_string(), serde_json::json!(event.tenant_id));
        if let Some(project_id) = event.payload.get("project_id") {
            map.insert("project_id".to_string(), project_id.clone());
        }
        if let Some(ticket_key) = event.payload.get("ticket_key") {
            map.insert("ticket_key".to_string(), ticket_key.clone());
        }
        if let Some(summary) = event.payload.get("summary") {
            map.insert("ticket_summary".to_string(), summary.clone());
        }
        merged
    }
}

#[async_trait]
impl EventPort for EventAutomationService {
    async fn ingest_event(&self, command: IngestEventCommand) -> Result<IngestEventResponse, DomainError> {
        if command.event_type.trim().is_empty() {
            return Err(DomainError::ValidationError("Event type cannot be empty".into()));
        }
        let event = IngestedEvent::new(NewIngestedEvent {
            tenant_id: command.tenant_id,
            event_type: command.event_type,
            payload: command.payload,
        });
        self.event_repository.save(&event).await?;
        info!(event_id = %event.id, event_type = %event.event_type, "Event ingested");

        let rules = self.rule_repository.find_active_by_tenant_and_event_type(event.tenant_id, &event.event_type).await?;
        let mut firings = Vec::with_capacity(rules.len());

        for rule in &rules {
            let matched = rule.matches(&event.payload);
            let firing = if !matched {
                RuleFiring::new(NewRuleFiring {
                    event_id: event.id,
                    rule_id: rule.id,
                    matched: false,
                    status: RuleFiringStatus::Skipped,
                    action_result: None,
                    error: None,
                })
            } else {
                match ActionSpec::parse(&rule.action) {
                    Ok(action) => match self.dispatch_action(&action, &event).await {
                        Ok(result) => {
                            info!(rule_id = %rule.id, "Automation rule fired successfully");
                            RuleFiring::new(NewRuleFiring {
                                event_id: event.id,
                                rule_id: rule.id,
                                matched: true,
                                status: RuleFiringStatus::Succeeded,
                                action_result: Some(result),
                                error: None,
                            })
                        }
                        Err(e) => {
                            error!(rule_id = %rule.id, error = %e, "Automation rule action failed");
                            RuleFiring::new(NewRuleFiring {
                                event_id: event.id,
                                rule_id: rule.id,
                                matched: true,
                                status: RuleFiringStatus::Failed,
                                action_result: None,
                                error: Some(e.to_string()),
                            })
                        }
                    },
                    Err(e) => RuleFiring::new(NewRuleFiring {
                        event_id: event.id,
                        rule_id: rule.id,
                        matched: true,
                        status: RuleFiringStatus::Failed,
                        action_result: None,
                        error: Some(e.to_string()),
                    }),
                }
            };
            self.firing_repository.save(&firing).await?;
            firings.push(firing);
        }

        Ok(IngestEventResponse {
            event: Self::to_event_response(&event),
            firings: firings.iter().map(Self::to_firing_response).collect(),
        })
    }

    async fn find_events_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<EventResponse>, DomainError> {
        let events = self.event_repository.find_by_tenant(tenant_id).await?;
        Ok(events.iter().map(Self::to_event_response).collect())
    }

    async fn find_firings_by_event(&self, event_id: Uuid) -> Result<Vec<RuleFiringResponse>, DomainError> {
        let firings = self.firing_repository.find_by_event_id(event_id).await?;
        Ok(firings.iter().map(Self::to_firing_response).collect())
    }
}
