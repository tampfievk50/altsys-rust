use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            success: true,
            code: 201,
            message: "Created".to_string(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: u16, message: String) -> Self {
        Self {
            success: false,
            code,
            message,
            data: None,
        }
    }

    pub fn no_content() -> Self {
        Self {
            success: true,
            code: 204,
            message: "No Content".to_string(),
            data: None,
        }
    }
}
