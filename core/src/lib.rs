use context::AppContext;
use events::AppEvent;
use std::path::PathBuf;

mod commands;
mod config;
mod context;
pub mod events;
pub mod extensions;
pub mod state;

#[derive(Clone)]
pub struct Core {
    context: AppContext,
}

impl Core {
    pub fn new() -> anyhow::Result<Self> {
        let context = AppContext::new()?;

        Self::register_core_commands(&context)?;

        Ok(Self { context })
    }

    fn register_core_commands(context: &AppContext) -> anyhow::Result<()> {
        let _registry = context
            .command_registry
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on command registry"))?;

        // TODO: Register handlers

        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Load extensions
        self.load_extensions().await?;

        self.start_event_processing().await;

        self.context
            .event_bus
            .publish(events::AppEvent::ApplicationStarted)?;

        Ok(())
    }

    async fn load_extensions(&mut self) -> anyhow::Result<()> {
        // For now, we'll use a default extensions directory
        // In the future, this should be configurable
        let extensions_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cognito")
            .join("extensions");

        // Create the extensions directory if it doesn't exist
        if !extensions_dir.exists() {
            std::fs::create_dir_all(&extensions_dir)?;
        }

        // Load all extensions in the directory
        if extensions_dir.exists() {
            let mut extension_manager = self.context.extension_manager.write()
                .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on extension manager"))?;

            // Iterate through directories in the extensions directory
            for entry in std::fs::read_dir(&extensions_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    // Try to load the extension
                    match extension_manager.load_extension(path).await {
                        Ok(id) => {
                            if let Some(manifest) = extension_manager.get_manifest(id) {
                                println!("Loaded extension: {} v{}", manifest.name, manifest.version);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to load extension from {:?}: {}", entry.path(), e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn start_event_processing(&self) {
        let mut receiver = self.context.event_bus.subscribe();
        let context = self.context.clone();

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                if let Err(e) = Self::handle_event(&context, event) {
                    eprintln!("Error handling event: {e}");
                }
            }
        });
    }

    fn handle_event(context: &AppContext, event: AppEvent) -> anyhow::Result<()> {
        if let AppEvent::QueryChanged(query) = event.clone() {
            context.handle_query(query)?;
        }

        Ok(())
    }

    pub fn context(&self) -> &AppContext {
        &self.context
    }
}
