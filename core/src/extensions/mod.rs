use serde::{Deserialize, Serialize};

mod extension;
mod wasm_runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub entry_point: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    FileSystem,
    Network,
    System,
    Clipboard,
}

#[derive(Debug, Clone)]
pub struct ExtensionManager {}
