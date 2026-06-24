use std::sync::Arc;
use crawler_domain::port::input::DownloadPort::DownloadPort;
use crawler_domain::port::input::UserPort::UserPort;

pub struct AppState {
    pub download_service: Arc<dyn DownloadPort>,
    pub user_service: Arc<dyn UserPort>,
}
