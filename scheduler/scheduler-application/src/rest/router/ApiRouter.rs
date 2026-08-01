use std::sync::Arc;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::state::AppState::AppState;
use crate::middleware::AuthMiddleware::require_auth;
use crate::middleware::CasbinMiddleware::require_permission;
use crate::rest::controller::{ExecutionController, SchedulerController};

pub fn create_router(state: Arc<AppState>) -> Router {
    // All routes require authentication + Casbin permission checks; the
    // scheduler service is a resource server that trusts tokens issued by sso.
    let management_routes = Router::new()
        // Schedulers
        .route("/schedulers", get(SchedulerController::get_all_schedulers).post(SchedulerController::create_scheduler))
        .route("/schedulers/{id}", get(SchedulerController::get_scheduler).put(SchedulerController::update_scheduler).delete(SchedulerController::delete_scheduler))
        .route("/schedulers/{id}/pause", post(SchedulerController::pause_scheduler))
        .route("/schedulers/{id}/resume", post(SchedulerController::resume_scheduler))
        .route("/schedulers/{id}/run", post(SchedulerController::run_scheduler))

        // Execution history
        .route("/schedulers/{id}/executions", get(ExecutionController::get_executions_by_scheduler))
        .route("/executions/{id}", get(ExecutionController::get_execution));

    // The auth/permission layers are applied to the outer router, *after*
    // nesting, so the Casbin check sees the full "/api/v1/..." request path.
    // Layering inside a nested sub-router instead would run after axum has
    // already stripped the nest prefix from the URI the middleware observes.
    Router::new()
        .nest("/api/v1", management_routes)
        .layer(middleware::from_fn_with_state(state.clone(), require_permission))
        .layer(middleware::from_fn(require_auth))
        .with_state(state)
}
