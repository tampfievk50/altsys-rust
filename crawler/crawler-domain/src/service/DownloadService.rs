use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};
use uuid::Uuid;

use crate::dto::DownloadVideoCommand::DownloadVideoCommand;
use crate::dto::DownloadVideoResponse::DownloadVideoResponse;
use crate::dto::VideoDownload::VideoDownload;
use crate::dto::VideoDownloadResponse::VideoDownloadResponse;
use crate::mapper::CrawlerDataMapper::CrawlerDataMapper;
use crate::port::input::DownloadPort::DownloadPort;
use crate::port::output::DownloadRepositoryPort::DownloadRepositoryPort;
use crate::port::output::VideoDownloaderPort::VideoDownloaderPort;
use crate::r#enum::DomainError::DomainError;

pub struct DownloadService {
    download_repository: Arc<dyn DownloadRepositoryPort>,
    video_downloader: Arc<dyn VideoDownloaderPort>,
    download_dir: String,
}

impl DownloadService {
    pub fn new(
        download_repository: Arc<dyn DownloadRepositoryPort>,
        video_downloader: Arc<dyn VideoDownloaderPort>,
        download_dir: String,
    ) -> Self {
        Self {
            download_repository,
            video_downloader,
            download_dir,
        }
    }

    fn extract_video_id(url: &str) -> Option<String> {
        // Handle youtu.be/VIDEO_ID
        if let Some(pos) = url.find("youtu.be/") {
            let id_start = pos + "youtu.be/".len();
            let id = url[id_start..]
                .split(['?', '&', '#'])
                .next()
                .unwrap_or("");
            if !id.is_empty() {
                return Some(id.to_string());
            }
        }

        // Handle youtube.com/watch?v=VIDEO_ID
        if let Some(pos) = url.find("v=") {
            let id_start = pos + 2;
            let id = url[id_start..]
                .split(['&', '#', '?'])
                .next()
                .unwrap_or("");
            if !id.is_empty() {
                return Some(id.to_string());
            }
        }

        // Handle youtube.com/shorts/VIDEO_ID or youtube.com/live/VIDEO_ID
        for prefix in &["/shorts/", "/live/"] {
            if let Some(pos) = url.find(prefix) {
                let id_start = pos + prefix.len();
                let id = url[id_start..]
                    .split(['?', '&', '#', '/'])
                    .next()
                    .unwrap_or("");
                if !id.is_empty() {
                    return Some(id.to_string());
                }
            }
        }

        None
    }
}

#[async_trait]
impl DownloadPort for DownloadService {
    async fn create_download(
        &self,
        command: DownloadVideoCommand,
    ) -> Result<DownloadVideoResponse, DomainError> {
        // Validate URL
        let url = command.url.trim().to_string();
        if url.is_empty() {
            return Err(DomainError::InvalidUrl("URL cannot be empty".to_string()));
        }

        let is_valid = url.contains("youtube.com/watch")
            || url.contains("youtu.be/")
            || url.contains("youtube.com/shorts/")
            || url.contains("youtube.com/live/");

        if !is_valid {
            return Err(DomainError::InvalidUrl(format!(
                "Not a valid YouTube URL: {}",
                url
            )));
        }

        // Extract video ID
        let video_id = Self::extract_video_id(&url);

        // Create download record
        let mut download = VideoDownload::new(url.clone(), video_id);

        // Save initial record
        self.download_repository.save(&download).await?;
        info!(download_id = %download.id, "Download record created, starting download");

        // Mark as downloading
        download.status = "DOWNLOADING".to_string();
        download.updated_at = Utc::now();
        self.download_repository.update(&download).await?;

        // Execute the download
        match self.video_downloader.download(&url, &self.download_dir).await {
            Ok(result) => {
                download.status = "COMPLETED".to_string();
                download.file_path = Some(result.file_path);
                download.file_size_bytes = Some(result.file_size_bytes);
                download.title = result.title;
                download.updated_at = Utc::now();
                self.download_repository.update(&download).await?;
                info!(download_id = %download.id, "Download completed successfully");
            }
            Err(e) => {
                error!(download_id = %download.id, error = %e, "Download failed");
                download.status = "FAILED".to_string();
                download.error_message = Some(e.to_string());
                download.updated_at = Utc::now();
                self.download_repository.update(&download).await?;
            }
        }

        let response = CrawlerDataMapper::to_download_video_response(
            &download,
            &format!("Download {}", download.status),
        );

        Ok(response)
    }

    async fn find_download_by_id(
        &self,
        id: Uuid,
    ) -> Result<VideoDownloadResponse, DomainError> {
        info!(download_id = %id, "Querying download by ID");

        let download = self
            .download_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Download not found with id: {}", id)))?;

        Ok(CrawlerDataMapper::to_video_download_response(&download))
    }

    async fn find_all_downloads(&self) -> Result<Vec<VideoDownloadResponse>, DomainError> {
        info!("Querying all downloads");

        let downloads = self.download_repository.find_all().await?;

        Ok(downloads
            .iter()
            .map(CrawlerDataMapper::to_video_download_response)
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_id_watch() {
        let id = DownloadService::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(id, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_short_url() {
        let id = DownloadService::extract_video_id("https://youtu.be/dQw4w9WgXcQ");
        assert_eq!(id, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_shorts() {
        let id = DownloadService::extract_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ");
        assert_eq!(id, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_invalid() {
        let id = DownloadService::extract_video_id("https://example.com");
        assert_eq!(id, None);
    }
}
