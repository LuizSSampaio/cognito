use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{commands::Action, state::SearchResult};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    // Core events
    QueryChanged(String),
    ResultsUpdated(Vec<SearchResult>),
    ItemSelected(Uuid),
    ItemActivated(Uuid, Action),

    // Navigation Events
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,

    // Mode Events
    EnterCommandMode,
    ExitToSearch,

    // System Events
    ApplicationStarted,
    ApplicationClosing,
}

pub type EventSender = broadcast::Sender<AppEvent>;
pub type EventReceiver = broadcast::Receiver<AppEvent>;

#[derive(Debug, Clone)]
pub(crate) struct EventBus {
    sender: EventSender,
}

impl EventBus {
    pub(crate) fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { sender }
    }

    pub fn subscribe(&self) -> EventReceiver {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: AppEvent) -> Result<(), broadcast::error::SendError<AppEvent>> {
        self.sender.send(event).map(|_| ())
    }

    pub fn sender(&self) -> EventSender {
        self.sender.clone()
    }
}
