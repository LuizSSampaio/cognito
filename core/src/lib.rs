use context::AppContext;
use events::AppEvent;

mod commands;
mod config;
mod context;
pub mod events;
mod extensions;
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
        let mut registry = context
            .command_registry
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock on state"))?;

        // TODO: Register handlers

        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.start_event_processing().await;

        self.context
            .event_bus
            .publish(events::AppEvent::ApplicationStarted)?;

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
