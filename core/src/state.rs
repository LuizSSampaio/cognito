use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::commands::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub query: String,
    pub items: Vec<Item>,
    pub mode: AppMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Search,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub actions: Vec<Action>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            query: Default::default(),
            items: Default::default(),
            mode: AppMode::Search,
        }
    }
}
