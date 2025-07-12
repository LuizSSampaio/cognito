use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::events::{EventBus, EventReceiver};

use super::ExtensionManifest;

#[async_trait]
pub trait Extension: Send + Sync {
    fn manifest(&self) -> &ExtensionManifest;
    async fn initialize(&mut self, context: ExtensionContext) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ExtensionContext {
    id: Uuid,
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
