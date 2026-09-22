use std::iter;

use ratatui::{
    text::{Line, Text},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
}

impl Command {
    pub fn to_list_item(&self) -> ListItem<'_> {
        let mut header: Vec<Line> = vec![];
        header.push(Line::from(self.name.to_string()));
        if !self.description.is_empty() {
            header.push(Line::from(self.description.to_string()));
        }

        let footer = Line::from("\n".to_string());

        let text: Vec<Line> = header.into_iter().chain(iter::once(footer)).collect();

        ListItem::new(Text::from(text))
    }
}
