use std::sync::Arc;

use async_trait::async_trait;

use sdlc_domain::dto::StartSdlcRunCommand::StartSdlcRunCommand;
use sdlc_domain::port::input::SdlcRunPort::SdlcRunPort;
use sdlc_domain::port::output::SdlcClientPort::SdlcClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the SDLC service with a direct call to the
/// merged `SdlcOrchestratorService`.
pub struct InProcessSdlcClient {
    sdlc_run_port: Arc<dyn SdlcRunPort>,
}

impl InProcessSdlcClient {
    pub fn new(sdlc_run_port: Arc<dyn SdlcRunPort>) -> Self {
        Self { sdlc_run_port }
    }
}

#[async_trait]
impl SdlcClientPort for InProcessSdlcClient {
    async fn start_run(&self, parameters: serde_json::Value) -> Result<serde_json::Value, DomainError> {
        let command: StartSdlcRunCommand = serde_json::from_value(parameters)
            .map_err(|e| DomainError::ValidationError(format!("start_sdlc_run action parameters do not match StartSdlcRunCommand: {}", e)))?;
        let run = self.sdlc_run_port.start_run(command).await?;
        serde_json::to_value(&run).map_err(|e| DomainError::InternalError(format!("Failed to serialize sdlc run: {}", e)))
    }
}
