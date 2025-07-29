use async_trait::async_trait;

use crate::events::{AppEvent, EventBus, EventReceiver};

use super::ExtensionManifest;

#[async_trait]
pub trait Extension: Send + Sync {
    fn publish_event(&self, event: AppEvent) -> anyhow::Result<()>;
    fn manifest(&self) -> &ExtensionManifest;
    fn initialize(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ExtensionApi {
    pub event_bus: EventBus,
}

impl Default for ExtensionApi {
    fn default() -> Self {
        Self {
            event_bus: EventBus::new(),
        }
    }
}

impl ExtensionApi {
    pub fn publish(&self, event: AppEvent) -> anyhow::Result<()> {
        self.event_bus
            .publish(event)
            .map_err(|e| anyhow::anyhow!("Failed to publish event: {}", e))
    }

    pub fn subscribe(&self) -> EventReceiver {
        self.event_bus.subscribe()
    }
}
