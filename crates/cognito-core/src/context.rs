use std::sync::{Arc, RwLock};

use anyhow::Ok;

use crate::{commands::CommandRegistry, events::EventBus, state::AppState};

#[derive(Clone)]
pub(crate) struct AppContext {
    pub state: Arc<RwLock<AppState>>,
    pub event_bus: EventBus,
    pub command_registry: Arc<RwLock<CommandRegistry>>,
}

impl AppContext {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            state: Arc::new(RwLock::new(AppState::default())),
            event_bus: EventBus::new(),
            command_registry: Arc::new(RwLock::new(CommandRegistry::default())),
        })
    }
}
