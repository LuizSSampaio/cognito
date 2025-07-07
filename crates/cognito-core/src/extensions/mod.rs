use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    commands::CommandRegistry,
    config::ConfigManager,
    events::{EventBus, EventReceiver},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub entry_point: String,
    pub permissions: Vec<Permission>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    FileSystem,
    Network,
    System,
    Clipboard,
}

#[async_trait]
pub trait Extension: Send + Sync {
    fn manifest(&self) -> &ExtensionManifest;
    async fn initialize(&mut self, context: ExtensionContext) -> anyhow::Result<()>;
    async fn shutdown(&mut self) -> anyhow::Result<()>;
}

pub struct ExtensionContext {
    id: String,
    api: Arc<ExtensionApi>,
}

pub struct ExtensionApi {
    event_bus: EventBus,
    config: Arc<RwLock<ConfigManager>>,
    command_registry: Arc<RwLock<CommandRegistry>>,
}

impl ExtensionApi {
    pub fn subscribe_event(&self) -> EventReceiver {
        todo!()
    }

    pub fn register_command(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn get_config(&self) -> Option<()> {
        todo!()
    }

    pub fn set_config(&self) -> anyhow::Result<()> {
        todo!()
    }
}
