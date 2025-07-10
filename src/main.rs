mod screen;
mod widget;

use core::{Core, events::AppEvent};

use iced::Element;
use log::error;
use screen::{Screen, search::Search};

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
    screen: Screen,
}

impl Default for Cognito {
    fn default() -> Self {
        let core = core::Core::new().unwrap();
        let screen = Screen::Search(Search::default());
        Self { core, screen }
    }
}

impl Cognito {
    pub fn update(&mut self, message: core::events::AppEvent) {
        match message {
            core::events::AppEvent::QueryChanged(query) => match &mut self.screen {
                Screen::Search(search) => {
                    if let Err(e) = self.core.context().handle_query(query) {
                        error!("Query handle failed: {e}");
                    }

                    search.query = self.core.context().get_query();
                }
            },
            core::events::AppEvent::ResultsUpdated(items) => match &mut self.screen {
                Screen::Search(search) => search.items = items,
            },
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, AppEvent> {
        match &self.screen {
            Screen::Search(search) => search.view(),
        }
    }
}
