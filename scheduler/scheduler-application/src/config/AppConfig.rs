use std::sync::Arc;
use casbin::Enforcer;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

use crate::state::AppState::AppState;

use scheduler_dataaccess::scheduler::adapter::SchedulerRepositoryImpl::SchedulerRepositoryImpl;
use scheduler_dataaccess::scheduler::repository::SchedulerSeaOrmRepository::SchedulerSeaOrmRepository;
use scheduler_dataaccess::job_execution::adapter::JobExecutionRepositoryImpl::JobExecutionRepositoryImpl;
use scheduler_dataaccess::job_execution::repository::JobExecutionSeaOrmRepository::JobExecutionSeaOrmRepository;

use scheduler_domain::port::input::SchedulerPort::SchedulerPort;
use scheduler_domain::port::input::SchedulerRunnerPort::SchedulerRunnerPort;
use scheduler_domain::port::input::ExecutionPort::ExecutionPort;

use scheduler_domain::service::SchedulerService::SchedulerService;
use scheduler_domain::service::SchedulerRunnerService::SchedulerRunnerService;
use scheduler_domain::service::ExecutionService::ExecutionService;

pub async fn create_app_state(db: DatabaseConnection, enforcer: Arc<RwLock<Enforcer>>) -> Arc<AppState> {
    // Scheduler wiring
    let scheduler_repo = Arc::new(SchedulerRepositoryImpl::new(SchedulerSeaOrmRepository::new(db.clone())));
    let scheduler_service = Arc::new(SchedulerService::new(scheduler_repo.clone())) as Arc<dyn SchedulerPort>;

    // Execution history wiring
    let execution_repo = Arc::new(JobExecutionRepositoryImpl::new(JobExecutionSeaOrmRepository::new(db.clone())));
    let execution_service = Arc::new(ExecutionService::new(execution_repo.clone())) as Arc<dyn ExecutionPort>;

    // Runner wiring (fires webhooks, drives both manual "run by id" and the scheduled tick)
    let scheduler_runner = Arc::new(SchedulerRunnerService::new(scheduler_repo, execution_repo)) as Arc<dyn SchedulerRunnerPort>;

    Arc::new(AppState {
        scheduler_service,
        scheduler_runner,
        execution_service,
        enforcer,
    })
}
