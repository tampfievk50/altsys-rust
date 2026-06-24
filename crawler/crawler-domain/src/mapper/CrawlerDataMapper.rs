use crate::dto::DownloadVideoResponse::DownloadVideoResponse;
use crate::dto::VideoDownload::VideoDownload;
use crate::dto::VideoDownloadResponse::VideoDownloadResponse;

pub struct CrawlerDataMapper;

impl CrawlerDataMapper {
    pub fn to_download_video_response(
        download: &VideoDownload,
        message: &str,
    ) -> DownloadVideoResponse {
        DownloadVideoResponse {
            id: download.id,
            video_url: download.video_url.clone(),
            status: download.status.clone(),
            message: message.to_string(),
            created_at: download.created_at,
            updated_at: download.updated_at,
            created_by: download.created_by,
            updated_by: download.updated_by,
        }
    }

    pub fn to_video_download_response(download: &VideoDownload) -> VideoDownloadResponse {
        VideoDownloadResponse {
            id: download.id,
            video_url: download.video_url.clone(),
            video_id: download.video_id.clone(),
            title: download.title.clone(),
            status: download.status.clone(),
            file_path: download.file_path.clone(),
            file_size_bytes: download.file_size_bytes,
            error_message: download.error_message.clone(),
            created_at: download.created_at,
            updated_at: download.updated_at,
            created_by: download.created_by,
            updated_by: download.updated_by,
        }
    }
}
