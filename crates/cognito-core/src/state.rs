use std::collections::HashMap;

use uuid::Uuid;

use crate::commands::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub selected_index: usize,
    pub mode: AppMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Search,
    Command,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub score: f64,
    pub actions: Vec<Action>,
    pub metadata: HashMap<String, String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            query: Default::default(),
            results: Default::default(),
            selected_index: Default::default(),
            mode: AppMode::Search,
        }
    }
}
