use core::{events::AppEvent, state::Item};

use iced::{
    Element,
    widget::{column, container, scrollable, text},
};

use crate::widget::search_bar::search_bar;

#[derive(Default, Clone)]
pub struct Search {
    pub query: String,
    pub items: Vec<Item>,
}

impl Search {
    pub fn view(&self) -> Element<'_, AppEvent> {
        let search_bar = search_bar(self.query.to_owned(), None);
        let items = scrollable(column(
            self.items
                .iter()
                .map(|item| self.render_item(item))
                .collect::<Vec<_>>(),
        ))
        .spacing(5);

        container(column![search_bar, items].spacing(10))
            .padding(10)
            .into()
    }

    fn render_item(&self, item: &Item) -> Element<'_, AppEvent> {
        text(item.title.to_owned()).into()
    }
}
