use std::str::FromStr;

use sea_orm::Set;

use sdlc_domain::dto::SdlcRun::SdlcRun;
use sdlc_domain::dto::SdlcRunStatus::SdlcRunStatus;

use crate::sdlc_run::entity::SdlcRunEntity;

pub struct SdlcRunDataMapper;

impl SdlcRunDataMapper {
    pub fn to_domain(entity: &SdlcRunEntity::Model) -> SdlcRun {
        SdlcRun {
            id: entity.id,
            tenant_id: entity.tenant_id,
            project_id: entity.project_id,
            ticket_key: entity.ticket_key.clone(),
            status: SdlcRunStatus::from_str(&entity.status).unwrap_or(SdlcRunStatus::Failed),
            current_step: entity.current_step.clone(),
            branch_name: entity.branch_name.clone(),
            pull_request_url: entity.pull_request_url.clone(),
            context: serde_json::from_str(&entity.context).unwrap_or_else(|_| serde_json::json!({})),
            error: entity.error.clone(),
            started_at: entity.started_at,
            completed_at: entity.completed_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }

    pub fn to_active_model(run: &SdlcRun) -> SdlcRunEntity::ActiveModel {
        SdlcRunEntity::ActiveModel {
            id: Set(run.id),
            tenant_id: Set(run.tenant_id),
            project_id: Set(run.project_id),
            ticket_key: Set(run.ticket_key.clone()),
            status: Set(run.status.to_string()),
            current_step: Set(run.current_step.clone()),
            branch_name: Set(run.branch_name.clone()),
            pull_request_url: Set(run.pull_request_url.clone()),
            context: Set(run.context.to_string()),
            error: Set(run.error.clone()),
            started_at: Set(run.started_at),
            completed_at: Set(run.completed_at),
            created_at: Set(run.created_at),
            updated_at: Set(run.updated_at),
        }
    }
}
