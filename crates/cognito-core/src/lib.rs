use context::AppContext;

mod commands;
mod config;
mod context;
mod events;
mod state;

#[derive(Clone)]
pub struct Core {
    context: AppContext,
}

impl Core {
    pub fn new() -> anyhow::Result<Self> {
        let context = AppContext::new()?;

        Ok(Self { context })
    }
}
