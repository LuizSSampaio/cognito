use cognito_core::{self as core, Core};

use iced::widget::{Column, column, text};

fn main() -> iced::Result {
    iced::application("Cognito", Cognito::update, Cognito::view)
        .resizable(false)
        .decorations(false)
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
    pub fn update(&mut self, _message: core::events::AppEvent) {}

    pub fn view(&self) -> Column<core::events::AppEvent> {
        column![text("Hello World").size(50)]
    }
}
