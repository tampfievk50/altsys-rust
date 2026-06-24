use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DownloadVideoResponse {
    pub id: Uuid,
    pub video_url: String,
    pub status: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
