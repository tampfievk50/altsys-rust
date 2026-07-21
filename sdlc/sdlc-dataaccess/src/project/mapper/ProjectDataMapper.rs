use sea_orm::Set;

use sdlc_domain::dto::Project::Project;

use crate::project::entity::ProjectEntity;

pub struct ProjectDataMapper;

impl ProjectDataMapper {
    pub fn to_domain(model: &ProjectEntity::Model) -> Project {
        Project {
            id: model.id,
            tenant_id: model.tenant_id,
            name: model.name.clone(),
            slug: model.slug.clone(),
            github_tool_id: model.github_tool_id,
            default_branch: model.default_branch.clone(),
            jira_tool_id: model.jira_tool_id,
            jira_last_synced_at: model.jira_last_synced_at,
            build_command: model.build_command.clone(),
            test_command: model.test_command.clone(),
            coding_standards: model.coding_standards.clone(),
            workflow_config: model.workflow_config.clone(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }

    pub fn to_active_model(project: &Project) -> ProjectEntity::ActiveModel {
        ProjectEntity::ActiveModel {
            id: Set(project.id),
            tenant_id: Set(project.tenant_id),
            name: Set(project.name.clone()),
            slug: Set(project.slug.clone()),
            github_tool_id: Set(project.github_tool_id),
            default_branch: Set(project.default_branch.clone()),
            jira_tool_id: Set(project.jira_tool_id),
            jira_last_synced_at: Set(project.jira_last_synced_at),
            build_command: Set(project.build_command.clone()),
            test_command: Set(project.test_command.clone()),
            coding_standards: Set(project.coding_standards.clone()),
            workflow_config: Set(project.workflow_config.clone()),
            is_active: Set(project.is_active),
            created_at: Set(project.created_at),
            updated_at: Set(project.updated_at),
            created_by: Set(project.created_by),
            updated_by: Set(project.updated_by),
        }
    }
}
