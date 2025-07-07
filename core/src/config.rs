use std::path::PathBuf;

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {}

impl Default for AppConfig {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: AppConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("cognito");

        std::fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");

        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            AppConfig::default()
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    pub fn get(&self) -> &AppConfig {
        &self.config
    }

    pub fn update<T>(&mut self, updater: T) -> anyhow::Result<()>
    where
        T: FnOnce(&mut AppConfig),
    {
        updater(&mut self.config);
        self.save()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(&self.config)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
}
