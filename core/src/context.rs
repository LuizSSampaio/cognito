use std::sync::{Arc, RwLock};

use crate::{
    commands::CommandRegistry, config::ConfigManager, events::EventBus,
    extensions::ExtensionManager, state::AppState,
};

#[derive(Clone)]
pub struct AppContext {
    pub state: Arc<RwLock<AppState>>,
    pub event_bus: EventBus,
    pub config: Arc<RwLock<ConfigManager>>,
    pub extension_manager: Arc<RwLock<ExtensionManager>>,
    pub command_registry: Arc<RwLock<CommandRegistry>>,
}

impl AppContext {
    pub(crate) fn new() -> anyhow::Result<Self> {
        Ok(Self {
            state: Arc::new(RwLock::new(AppState::default())),
            event_bus: EventBus::new(),
            config: Arc::new(RwLock::new(ConfigManager::new()?)),
            extension_manager: Arc::new(RwLock::new(ExtensionManager::default())),
            command_registry: Arc::new(RwLock::new(CommandRegistry::default())),
        })
    }

    pub fn get_query(&self) -> String {
        match self.state.read() {
            Ok(state) => state.query.clone(),
            Err(_) => String::new(),
        }
    }

    pub fn handle_query(&self, query: String) -> anyhow::Result<()> {
        {
            let mut state = self
                .state
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on state"))?;
            state.query = query.clone();
        }

        {
            let mut state = self
                .state
                .write()
                .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on state"))?;
            state.items = Vec::new();
        }

        Ok(())
    }
}
