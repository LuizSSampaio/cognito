use std::{collections::HashMap, path::PathBuf};

use extension::Extension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;

mod extension;
mod wasm_extension;

use wasm_extension::WasmExtension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub entry_file: String,
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
    pub fn new() -> Result<Self> {
        Ok(Self {
            extensions: HashMap::new(),
        })
    }
    
    pub async fn load_extension(&mut self, path: PathBuf) -> anyhow::Result<Uuid> {
        let manifest: ExtensionManifest = {
            let content = std::fs::read_to_string(path.clone().join("manifest.toml"))?;
            toml::from_str(&content)?
        };

        let id = Uuid::new_v4();
        
        // Create and initialize the WASM extension
        let mut wasm_extension = WasmExtension::new(id, manifest, path);
        wasm_extension.initialize().await?;
        
        self.extensions.insert(id, Box::new(wasm_extension));
        
        Ok(id)
    }

    pub fn unload_extension(&mut self, id: Uuid) -> anyhow::Result<()> {
        match self.extensions.remove(&id) {
            Some(_) => Ok(()),
            None => anyhow::bail!("Couldn't find an extension with ID: {}", id),
        }
    }

    pub fn get_manifest(&self, id: Uuid) -> Option<&ExtensionManifest> {
        if let Some(extension) = self.extensions.get(&id) {
            return Some(extension.manifest());
        }

        None
    }
}