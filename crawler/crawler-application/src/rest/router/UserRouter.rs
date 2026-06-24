use std::sync::Arc;

use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::rest::controller::UserController::{
    create_user, delete_user, get_user_by_id, list_users, update_user,
};
use crate::state::AppState::AppState;

pub struct UserRouter;

impl UserRouter {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/api/v1/users", post(create_user))
            .route("/api/v1/users", get(list_users))
            .route("/api/v1/users/{id}", get(get_user_by_id))
            .route("/api/v1/users/{id}", put(update_user))
            .route("/api/v1/users/{id}", delete(delete_user))
    }
}
