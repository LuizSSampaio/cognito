use std::sync::Arc;

use async_trait::async_trait;

use crate::events::{AppEvent, EventBus, EventReceiver};

use super::ExtensionManifest;

#[async_trait]
pub trait Extension: Send + Sync {
    fn load(path: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn publish_event(&self, event: AppEvent) -> anyhow::Result<()>;
    fn manifest(&self) -> &ExtensionManifest;
    async fn initialize(&mut self, context: ExtensionContext) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ExtensionContext {
    pub api: Arc<ExtensionApi>,
}

#[derive(Debug, Clone)]
pub struct ExtensionApi {
    event_bus: EventBus,
}

impl ExtensionApi {
    pub fn subscribe_event(&self) -> EventReceiver {
        todo!()
    }
}
