mod widget;

use core::{Core, events::AppEvent};

use iced::widget::{Column, column};
use log::error;
use widget::search::search_bar;

fn main() -> iced::Result {
    iced::application("Cognito", Cognito::update, Cognito::view)
        .resizable(false)
        .decorations(false)
        .centered()
        .level(iced::window::Level::AlwaysOnTop)
        .exit_on_close_request(false)
        .run()
}

struct Cognito {
    core: Core,
}

impl Default for Cognito {
    fn default() -> Self {
        let core = core::Core::new().unwrap();
        Self { core }
    }
}

impl Cognito {
    pub fn update(&mut self, message: core::events::AppEvent) {
        if let AppEvent::QueryChanged(new) = message {
            if let Err(e) = self.core.context().handle_query(new) {
                error!("Query handle failed: {e}");
            }
        }
    }

    pub fn view(&self) -> Column<core::events::AppEvent> {
        column![search_bar(self.core.context().get_query(), None)]
    }
}
