use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::rest::payload::SchedulerPayloads::{CreateSchedulerRequest, UpdateSchedulerRequest};
use crate::rest::response::ApiResponse::ApiResponse;

use scheduler_domain::dto::SchedulerResponse::SchedulerResponse;
use scheduler_domain::dto::ExecutionResponse::ExecutionResponse;

use crate::rest::controller::{ExecutionController, SchedulerController};

#[derive(OpenApi)]
#[openapi(
    paths(
        SchedulerController::create_scheduler,
        SchedulerController::get_all_schedulers,
        SchedulerController::get_scheduler,
        SchedulerController::update_scheduler,
        SchedulerController::delete_scheduler,
        SchedulerController::pause_scheduler,
        SchedulerController::resume_scheduler,
        SchedulerController::run_scheduler,
        ExecutionController::get_executions_by_scheduler,
        ExecutionController::get_execution,
    ),
    components(
        schemas(
            CreateSchedulerRequest,
            UpdateSchedulerRequest,
            ApiResponse<SchedulerResponse>,
            ApiResponse<Vec<SchedulerResponse>>,
            ApiResponse<ExecutionResponse>,
            ApiResponse<Vec<ExecutionResponse>>,
            SchedulerResponse,
            ExecutionResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Schedulers", description = "Scheduler management APIs (CRUD, pause/resume, run-by-id)"),
        (name = "Executions", description = "Scheduler execution history APIs"),
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
