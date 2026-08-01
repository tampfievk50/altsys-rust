use std::sync::Arc;
use std::time::Duration;

use tracing::error;

use crate::state::AppState::AppState;

/// Background poller that plays the role of Quartz's scheduler thread: on
/// every tick it asks the runner to fire whatever schedulers are due.
pub fn start(state: Arc<AppState>) {
    let poll_interval_seconds: u64 = std::env::var("SCHEDULER_POLL_INTERVAL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(poll_interval_seconds));
        loop {
            interval.tick().await;
            if let Err(e) = state.scheduler_runner.tick().await {
                error!(error = %e, "Scheduler engine tick failed");
            }
        }
    });
}
