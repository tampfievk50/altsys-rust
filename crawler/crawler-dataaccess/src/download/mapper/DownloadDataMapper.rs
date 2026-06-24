use crawler_domain::dto::VideoDownload::VideoDownload;
use sea_orm::Set;

use crate::download::entity::DownloadEntity::{ActiveModel, Model};

pub struct DownloadDataMapper;

impl DownloadDataMapper {
    pub fn to_active_model(download: &VideoDownload) -> ActiveModel {
        ActiveModel {
            id: Set(download.id),
            video_url: Set(download.video_url.clone()),
            video_id: Set(download.video_id.clone()),
            title: Set(download.title.clone()),
            status: Set(download.status.clone()),
            file_path: Set(download.file_path.clone()),
            file_size_bytes: Set(download.file_size_bytes.map(|s| s as i64)),
            error_message: Set(download.error_message.clone()),
            created_at: Set(download.created_at),
            updated_at: Set(download.updated_at),
            created_by: Set(download.created_by),
            updated_by: Set(download.updated_by),
        }
    }

    pub fn to_domain(model: &Model) -> VideoDownload {
        VideoDownload {
            id: model.id,
            video_url: model.video_url.clone(),
            video_id: model.video_id.clone(),
            title: model.title.clone(),
            status: model.status.clone(),
            file_path: model.file_path.clone(),
            file_size_bytes: model.file_size_bytes.map(|s| s as u64),
            error_message: model.error_message.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
            created_by: model.created_by,
            updated_by: model.updated_by,
        }
    }
}
