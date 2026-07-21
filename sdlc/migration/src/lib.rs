pub use sea_orm_migration::prelude::*;

mod m20260701_000001_create_projects_table;
mod m20260701_000002_create_credentials_table;
mod m20260701_000003_create_models_table;
mod m20260701_000004_create_prompts_table;
mod m20260702_000001_create_tools_table;
mod m20260702_000001_create_knowledge_items_table;
mod m20260703_000001_convert_embedding_to_pgvector;
mod m20260702_000001_create_workflow_definitions_table;
mod m20260702_000002_create_workflow_executions_table;
mod m20260702_000003_create_workflow_node_executions_table;
mod m20260704_000001_create_agents_table;
mod m20260704_000002_create_agent_executions_table;
mod m20260705_000001_create_sdlc_runs_table;
mod m20260705_000002_create_sdlc_step_executions_table;
mod m20260706_000001_create_plugins_table;
mod m20260706_000002_create_automation_rules_table;
mod m20260706_000003_create_workflow_templates_table;
mod m20260706_000004_create_events_table;
mod m20260706_000005_create_rule_firings_table;
mod m20260707_000001_create_skills_table;
mod m20260707_000002_create_agent_skills_table;
mod m20260708_000001_add_jira_last_synced_at_to_projects_table;
mod m20260709_000001_create_task_overrides_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260701_000001_create_projects_table::Migration),
            Box::new(m20260701_000002_create_credentials_table::Migration),
            Box::new(m20260701_000003_create_models_table::Migration),
            Box::new(m20260701_000004_create_prompts_table::Migration),
            Box::new(m20260702_000001_create_tools_table::Migration),
            Box::new(m20260702_000001_create_knowledge_items_table::Migration),
            Box::new(m20260703_000001_convert_embedding_to_pgvector::Migration),
            Box::new(m20260702_000001_create_workflow_definitions_table::Migration),
            Box::new(m20260702_000002_create_workflow_executions_table::Migration),
            Box::new(m20260702_000003_create_workflow_node_executions_table::Migration),
            Box::new(m20260704_000001_create_agents_table::Migration),
            Box::new(m20260704_000002_create_agent_executions_table::Migration),
            Box::new(m20260705_000001_create_sdlc_runs_table::Migration),
            Box::new(m20260705_000002_create_sdlc_step_executions_table::Migration),
            Box::new(m20260706_000001_create_plugins_table::Migration),
            Box::new(m20260706_000002_create_automation_rules_table::Migration),
            Box::new(m20260706_000003_create_workflow_templates_table::Migration),
            Box::new(m20260706_000004_create_events_table::Migration),
            Box::new(m20260706_000005_create_rule_firings_table::Migration),
            Box::new(m20260707_000001_create_skills_table::Migration),
            Box::new(m20260707_000002_create_agent_skills_table::Migration),
            Box::new(m20260708_000001_add_jira_last_synced_at_to_projects_table::Migration),
            Box::new(m20260709_000001_create_task_overrides_table::Migration),
        ]
    }
}
