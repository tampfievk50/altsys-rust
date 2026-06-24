use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub status: String,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "SUCCESS".to_string(),
            code: 200,
            data: Some(data),
            message: None,
            timestamp: Utc::now(),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            status: "ERROR".to_string(),
            code,
            data: None,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }
}
