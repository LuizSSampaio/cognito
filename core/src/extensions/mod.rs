use std::{collections::HashMap, path::PathBuf};

use extension::Extension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_runtime::WasmExtension;

use crate::events::AppEvent;

mod extension;
mod wasm_runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub entry_file: String,
    pub extension_type: ExtensionType,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExtensionType {
    WebAssembly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    FileSystem,
    Network,
    System,
    Clipboard,
}

#[derive(Default)]
pub struct ExtensionManager {
    extensions: HashMap<Uuid, Box<dyn Extension>>,
}

impl ExtensionManager {
    pub fn load_extension(&mut self, path: PathBuf) -> anyhow::Result<Uuid> {
        let manifest: ExtensionManifest = {
            let content = std::fs::read_to_string(path.clone().join("manifest.toml"))?;
            toml::from_str(&content)?
        };

        let id = Uuid::new_v4();
        match manifest.extension_type {
            ExtensionType::WebAssembly => {
                let wasm_file_path = {
                    if manifest.entry_file.ends_with(".wat") {
                        manifest.entry_file.clone()
                    } else {
                        format!("{}.wat", manifest.entry_file)
                    }
                };

                let mut extension =
                    WasmExtension::load(manifest, path.join(wasm_file_path).as_path())?;
                extension.initialize()?;

                self.extensions.insert(id, Box::new(extension));
            }
        }

        Ok(id)
    }

    pub fn unload_extension(&mut self, id: Uuid) -> anyhow::Result<()> {
        match self.extensions.remove(&id) {
            Some(_) => Ok(()),
            None => anyhow::bail!("Couldn't find an extension with ID: {}", id),
        }
    }

    pub fn handle_event(&self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::QueryChanged(query) => {
                for extension in self.extensions.values() {
                    extension.publish_event(AppEvent::QueryChanged(query.clone()))?;
                }

                Ok(())
            }
            AppEvent::ItemActivated(uuid, index) => {
                for extension in self.extensions.values() {
                    if extension.get_items_ids()?.contains(&uuid) {
                        extension.publish_event(AppEvent::ItemActivated(uuid, index))?;
                        break;
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn get_manifest(&self, id: Uuid) -> Option<&ExtensionManifest> {
        if let Some(extension) = self.extensions.get(&id) {
            return Some(extension.manifest());
        }

        None
    }
}
