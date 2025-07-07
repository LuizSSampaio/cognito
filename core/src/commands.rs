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
pub struct CommandRegistry {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppContext;

    struct MockCommandHandler {
        can_handle_result: bool,
        execute_result: CommandResult,
    }

    #[async_trait]
    impl CommandHandler for MockCommandHandler {
        async fn execute(
            &self,
            _command: CommandType,
            _context: &AppContext,
        ) -> anyhow::Result<CommandResult> {
            Ok(self.execute_result.clone())
        }

        fn can_handle(&self, _command: &CommandType) -> bool {
            self.can_handle_result
        }
    }

    async fn create_mock_context() -> AppContext {
        AppContext::new().expect("Create AppContext mock")
    }

    #[tokio::test]
    async fn test_command_registry_execute_success() {
        let mut registry = CommandRegistry::default();
        let handler = Box::new(MockCommandHandler {
            can_handle_result: true,
            execute_result: CommandResult::Success,
        });

        registry.register_handler("test_handler".to_string(), handler);

        let command = CommandType::OpenFile {
            path: "/test/file".to_string(),
        };
        let context = create_mock_context().await;

        let result = registry.execute(command, &context).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), CommandResult::Success);
    }

    #[tokio::test]
    async fn test_command_registry_execute_no_handler() {
        let registry = CommandRegistry::default();
        let command = CommandType::OpenFile {
            path: "/test/file".to_string(),
        };
        let context = create_mock_context().await;

        let result = registry.execute(command, &context).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No handler found for command")
        );
    }
}
