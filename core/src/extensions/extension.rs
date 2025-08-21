use async_trait::async_trait;

use super::ExtensionManifest;

#[async_trait]
pub trait Extension: Send + Sync {
    fn manifest(&self) -> &ExtensionManifest;
    fn initialize(&mut self) -> anyhow::Result<()>;
}
