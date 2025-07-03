use anyhow::Ok;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AppContext {}

impl AppContext {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
