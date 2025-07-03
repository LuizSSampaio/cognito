use context::AppContext;

mod context;
mod state;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Core {
    context: AppContext,
}

impl Core {
    pub async fn new() -> anyhow::Result<Self> {
        let context = AppContext::new().await?;

        Ok(Self { context })
    }
}
