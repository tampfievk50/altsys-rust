use utoipa::OpenApi;

use crate::rest::payload::CreateUserPayload::CreateUserPayload;
use crate::rest::payload::DownloadVideoPayload::DownloadVideoPayload;
use crate::rest::payload::UpdateUserPayload::UpdateUserPayload;

use crawler_domain::dto::DownloadVideoResponse::DownloadVideoResponse;
use crawler_domain::dto::UserResponse::UserResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::rest::controller::UserController::create_user,
        crate::rest::controller::UserController::list_users,
        crate::rest::controller::UserController::get_user_by_id,
        crate::rest::controller::UserController::update_user,
        crate::rest::controller::UserController::delete_user,
        crate::rest::controller::DownloadController::create_download,
        crate::rest::controller::DownloadController::list_downloads,
        crate::rest::controller::DownloadController::get_download_by_id,
    ),
    components(
        schemas(
            CreateUserPayload,
            UpdateUserPayload,
            UserResponse,
            DownloadVideoPayload,
            DownloadVideoResponse,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Downloads", description = "Video download management endpoints"),
    ),
    info(
        title = "Crawler API",
        version = "1.0.0",
        description = "REST API for the Crawler service — manage users and video downloads."
    )
)]
pub struct ApiDoc;
