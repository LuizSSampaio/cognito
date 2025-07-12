use std::collections::HashMap;

use extension::Extension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::events::AppEvent;

mod extension;
mod wasm_runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
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

pub struct ExtensionManager {
    extensions: HashMap<Uuid, Box<dyn Extension>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn load_extension(&mut self, path: &str) -> anyhow::Result<Uuid> {
        todo!()
    }

    pub fn unload_extension(&mut self, id: Uuid) -> anyhow::Result<()> {
        todo!()
    }

    pub fn handle_event(&self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::QueryChanged(query) => {
                for extension in self.extensions.values() {
                    extension.publish_event(AppEvent::QueryChanged(query.clone()))?;
                }

                Ok(())
            }
            AppEvent::ItemActivated(uuid, _) => todo!(),
            _ => Ok(()),
        }
    }

    pub fn get_manifest(&self, id: Uuid) -> Option<ExtensionManifest> {
        if let Some(extension) = self.extensions.get(&id) {
            return Some(extension.manifest().clone());
        }

        None
    }
}
