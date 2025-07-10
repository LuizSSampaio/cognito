use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{commands::Action, state::Item};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    // Core events
    QueryChanged(String),
    ResultsUpdated(Vec<Item>),
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
pub struct EventBus {
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

    pub fn publish(
        &self,
        event: AppEvent,
    ) -> Result<(), Box<broadcast::error::SendError<AppEvent>>> {
        match self.sender.send(event) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn sender(&self) -> EventSender {
        self.sender.clone()
    }
}
