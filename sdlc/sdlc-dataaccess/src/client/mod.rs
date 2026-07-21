#![allow(non_snake_case)]

pub mod PluginHttpDispatcher;

// The former sdlc/automation services called platform/tools/knowledge/
// workflow/agents/sdlc over HTTP with self-minted JWTs. Now that everything
// lives in one process, those output ports get in-process adapters instead.
pub mod InProcessPlatformClient;
pub mod InProcessSdlcToolsClient;
pub mod InProcessKnowledgeClient;
pub mod InProcessAgentsClient;
pub mod InProcessWorkflowClient;
pub mod InProcessSdlcClient;
pub mod InProcessAutomationToolsClient;
