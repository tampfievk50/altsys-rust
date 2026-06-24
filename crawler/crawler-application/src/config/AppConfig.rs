use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::state::AppState::AppState;
use crawler_dataaccess::download::adapter::DownloadRepositoryImpl::DownloadRepositoryImpl;
use crawler_dataaccess::download::repository::DownloadSeaOrmRepository::DownloadSeaOrmRepository;
use crawler_dataaccess::user::adapter::UserRepositoryImpl::UserRepositoryImpl;
use crawler_dataaccess::user::repository::UserSeaOrmRepository::UserSeaOrmRepository;
use crawler_domain::port::input::DownloadPort::DownloadPort;
use crawler_domain::port::input::UserPort::UserPort;
use crawler_domain::service::DownloadService::DownloadService;
use crawler_domain::service::UserService::UserService;
use crawler_domain::service::YtDlpDownloaderService::YtDlpDownloaderService;

pub fn create_app_state(db: DatabaseConnection, download_dir: String) -> Arc<AppState> {
    // Download wiring
    let sea_orm_repo = DownloadSeaOrmRepository::new(db.clone());
    let download_repository = Arc::new(DownloadRepositoryImpl::new(sea_orm_repo));
    let video_downloader = Arc::new(YtDlpDownloaderService::new());
    let download_service = Arc::new(DownloadService::new(
        download_repository,
        video_downloader,
        download_dir,
    )) as Arc<dyn DownloadPort>;

    // User wiring
    let user_sea_orm_repo = UserSeaOrmRepository::new(db);
    let user_repository = Arc::new(UserRepositoryImpl::new(user_sea_orm_repo));
    let user_service = Arc::new(UserService::new(user_repository)) as Arc<dyn UserPort>;

    Arc::new(AppState {
        download_service,
        user_service,
    })
}
