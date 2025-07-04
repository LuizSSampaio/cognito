use std::sync::{Arc, RwLock};

use anyhow::Ok;

use crate::{commands::CommandRegistry, config::ConfigManager, events::EventBus, state::AppState};

#[derive(Clone)]
pub(crate) struct AppContext {
    pub state: Arc<RwLock<AppState>>,
    pub event_bus: EventBus,
    pub config: Arc<RwLock<ConfigManager>>,
    pub command_registry: Arc<RwLock<CommandRegistry>>,
}

impl AppContext {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            state: Arc::new(RwLock::new(AppState::default())),
            event_bus: EventBus::new(),
            config: Arc::new(RwLock::new(ConfigManager::new()?)),
            command_registry: Arc::new(RwLock::new(CommandRegistry::default())),
        })
    }
}
