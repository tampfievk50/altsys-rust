use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Simple data model representing a video download record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownload {
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

impl VideoDownload {
    pub fn new(video_url: String, video_id: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            video_url,
            video_id,
            title: None,
            status: "PENDING".to_string(),
            file_path: None,
            file_size_bytes: None,
            error_message: None,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
