use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{AppContext, state::SearchResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub command_type: CommandType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CommandType {
    OpenApplication { path: String },
    OpenFile { path: String },
    OpenUrl { path: String },
    RunScript { script: String, args: Vec<String> },
    CopyToClipboard { text: String },
}

#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn execute(
        &self,
        command: CommandType,
        context: &AppContext,
    ) -> anyhow::Result<CommandResult>;
    fn can_handle(&self, command: &CommandType) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    Success,
    ShowResults(Vec<SearchResult>),
    ShowNotification(String),
    Error(String),
}

#[derive(Default)]
pub(crate) struct CommandRegistry {
    handlers: HashMap<String, Box<dyn CommandHandler>>,
}

impl CommandRegistry {
    pub fn register_handler(&mut self, id: String, handler: Box<dyn CommandHandler>) {
        self.handlers.insert(id, handler);
    }

    pub async fn execute(
        &self,
        command: CommandType,
        context: &AppContext,
    ) -> anyhow::Result<CommandResult> {
        for handler in self.handlers.values() {
            if handler.can_handle(&command) {
                return handler.execute(command, context).await;
            }
        }

        Err(anyhow::anyhow!(
            "No handler found for command: {:?}",
            command
        ))
    }
}
