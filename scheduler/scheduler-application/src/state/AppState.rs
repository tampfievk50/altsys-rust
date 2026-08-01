use std::sync::Arc;
use casbin::Enforcer;
use tokio::sync::RwLock;

use scheduler_domain::port::input::SchedulerPort::SchedulerPort;
use scheduler_domain::port::input::SchedulerRunnerPort::SchedulerRunnerPort;
use scheduler_domain::port::input::ExecutionPort::ExecutionPort;

pub struct AppState {
    pub scheduler_service: Arc<dyn SchedulerPort>,
    pub scheduler_runner: Arc<dyn SchedulerRunnerPort>,
    pub execution_service: Arc<dyn ExecutionPort>,
    pub enforcer: Arc<RwLock<Enforcer>>,
}
