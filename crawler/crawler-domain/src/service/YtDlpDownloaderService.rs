use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;
use tracing::{error, info};

use crate::port::output::VideoDownloaderPort::{DownloadResult, VideoDownloaderPort};
use crate::r#enum::DomainError::DomainError;

pub struct YtDlpDownloaderService;

impl YtDlpDownloaderService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YtDlpDownloaderService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VideoDownloaderPort for YtDlpDownloaderService {
    async fn download(&self, url: &str, download_dir: &str) -> Result<DownloadResult, DomainError> {
        tokio::fs::create_dir_all(download_dir).await.map_err(|e| {
            DomainError::DownloadFailed(format!("Failed to create download directory: {}", e))
        })?;

        let output_template = format!("{}/%(title)s.%(ext)s", download_dir);

        info!(url = %url, output = %output_template, "Starting yt-dlp download");

        // Get the title
        let title_output = Command::new("yt-dlp")
            .args(["--print", "%(title)s", "--no-download", url])
            .output()
            .await
            .map_err(|e| {
                DomainError::DownloadFailed(format!(
                    "Failed to execute yt-dlp (is it installed?): {}",
                    e
                ))
            })?;

        let title = if title_output.status.success() {
            let t = String::from_utf8_lossy(&title_output.stdout)
                .trim()
                .to_string();
            if t.is_empty() { None } else { Some(t) }
        } else {
            None
        };

        // Download the video
        let output = Command::new("yt-dlp")
            .args([
                "-f", "best",
                "--no-playlist",
                "--restrict-filenames",
                "-o", &output_template,
                "--print", "after_move:filepath",
                url,
            ])
            .output()
            .await
            .map_err(|e| {
                DomainError::DownloadFailed(format!(
                    "Failed to execute yt-dlp (is it installed?): {}",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(stderr = %stderr, "yt-dlp failed");
            return Err(DomainError::DownloadFailed(format!(
                "yt-dlp exited with error: {}",
                stderr.trim()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let file_path = stdout
            .lines()
            .last()
            .map(|l| l.trim().to_string())
            .unwrap_or_default();

        if file_path.is_empty() {
            return Err(DomainError::DownloadFailed(
                "yt-dlp did not output a file path".to_string(),
            ));
        }

        let metadata = tokio::fs::metadata(&file_path).await.map_err(|e| {
            DomainError::DownloadFailed(format!(
                "Downloaded file not found at {}: {}",
                file_path, e
            ))
        })?;

        let file_size_bytes = metadata.len();

        info!(
            file_path = %file_path,
            file_size_bytes = file_size_bytes,
            title = ?title,
            "yt-dlp download completed"
        );

        let display_path = Path::new(&file_path)
            .strip_prefix(download_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(file_path);

        Ok(DownloadResult {
            file_path: display_path,
            file_size_bytes,
            title,
        })
    }
}
