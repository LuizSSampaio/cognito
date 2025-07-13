use std::collections::HashSet;

use async_trait::async_trait;
use uuid::Uuid;

use crate::events::{AppEvent, EventBus, EventReceiver};

use super::ExtensionManifest;

#[async_trait]
pub trait Extension: Send + Sync {
    fn publish_event(&self, event: AppEvent) -> anyhow::Result<()>;
    fn get_items_ids(&self) -> anyhow::Result<&HashSet<Uuid>>;
    fn manifest(&self) -> &ExtensionManifest;
    fn initialize(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ExtensionApi {
    pub event_bus: EventBus,
    pub item_ids: HashSet<Uuid>,
}

impl Default for ExtensionApi {
    fn default() -> Self {
        Self {
            event_bus: EventBus::new(),
            item_ids: Default::default(),
        }
    }
}

impl ExtensionApi {
    pub fn subscribe_event(&self) -> EventReceiver {
        self.event_bus.subscribe()
    }

    pub fn add_item(&mut self) -> Uuid {
        todo!()
    }

    pub fn remove_item(&mut self, id: Uuid) -> anyhow::Result<()> {
        if self.item_ids.remove(&id) {
            return Ok(());
        }

        anyhow::bail!("Couldn't find an item with ID: {}", id)
    }

    pub fn get_items(&self) -> HashSet<Uuid> {
        self.item_ids.clone()
    }
}
