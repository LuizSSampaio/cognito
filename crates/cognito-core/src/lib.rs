use context::AppContext;

mod commands;
mod context;
mod state;

#[derive(Clone)]
pub struct Core {
    context: AppContext,
}

impl Core {
    pub async fn new() -> anyhow::Result<Self> {
        let context = AppContext::new().await?;

        Ok(Self { context })
    }
}
