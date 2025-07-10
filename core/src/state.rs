use uuid::Uuid;

use crate::commands::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub query: String,
    pub items: Vec<Item>,
    pub selected_index: usize,
    pub mode: AppMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Search,
    Command,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub score: i32,
    pub actions: Vec<Action>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            query: Default::default(),
            items: Default::default(),
            selected_index: Default::default(),
            mode: AppMode::Search,
        }
    }
}
