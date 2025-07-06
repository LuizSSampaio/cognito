use cognito_core::events::AppEvent;

use iced::{Element, widget::text_input};

pub fn search_bar(query: String, place_holder: Option<&str>) -> Element<'_, AppEvent> {
    text_input(place_holder.unwrap_or("Type to search..."), &query)
        .on_input(AppEvent::QueryChanged)
        .into()
}
