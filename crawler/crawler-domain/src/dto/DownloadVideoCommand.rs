use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DownloadVideoCommand {
    pub url: String,
}
