use std::sync::{Arc, RwLock};

use anyhow::Ok;

use crate::commands::CommandRegistry;

#[derive(Clone)]
pub(crate) struct AppContext {
    pub command_registry: Arc<RwLock<CommandRegistry>>,
}

impl AppContext {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            command_registry: Arc::new(RwLock::new(CommandRegistry::default())),
        })
    }
}
