use std::sync::{Arc, RwLock};

use crate::{commands::CommandRegistry, config::ConfigManager, events::EventBus, state::AppState};

#[derive(Clone)]
pub struct AppContext {
    pub state: Arc<RwLock<AppState>>,
    pub event_bus: EventBus,
    pub config: Arc<RwLock<ConfigManager>>,
    pub command_registry: Arc<RwLock<CommandRegistry>>,
}

impl AppContext {
    pub(crate) fn new() -> anyhow::Result<Self> {
        Ok(Self {
            state: Arc::new(RwLock::new(AppState::default())),
            event_bus: EventBus::new(),
            config: Arc::new(RwLock::new(ConfigManager::new()?)),
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
            state.selected_index = 0;
        }

        self.event_bus
            .publish(crate::events::AppEvent::QueryChanged(query))?;

        Ok(())
    }

    pub async fn active_selected(&self) -> anyhow::Result<()> {
        let (result_id, action) = {
            let state = self
                .state
                .read()
                .map_err(|_| anyhow::anyhow!("Failed to acquire read lock on state"))?;

            let result = state
                .items
                .get(state.selected_index)
                .ok_or_else(|| anyhow::anyhow!("No item selected"))?;
            let action = result
                .actions
                .first()
                .ok_or_else(|| anyhow::anyhow!("No action available"))?;

            (result.id, action.clone())
        };

        let command_registry = self
            .command_registry
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to acquire read lock on state"))?;
        command_registry
            .execute(action.command_type.to_owned(), self)
            .await?;

        self.event_bus
            .publish(crate::events::AppEvent::ItemActivated(result_id, action))?;

        Ok(())
    }
}
