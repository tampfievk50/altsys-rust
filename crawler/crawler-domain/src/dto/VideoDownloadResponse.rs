use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct VideoDownloadResponse {
    pub id: Uuid,
    pub video_url: String,
    pub video_id: Option<String>,
    pub title: Option<String>,
    pub status: String,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
