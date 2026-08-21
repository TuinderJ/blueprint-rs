use std::iter;

use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Enum {
    pub description: String,
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub note: String,
}

impl Variant {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            note: "".to_string(),
        }
    }
}

impl Enum {
    pub fn new() -> Self {
        Self {
            description: "".to_string(),
            name: "".to_string(),
            variants: vec![],
        }
    }

    pub fn empty() -> Self {
        Self {
            description: "".to_string(),
            name: "".to_string(),
            variants: vec![],
        }
    }

    pub fn to_list_item(&self) -> ListItem<'_> {
        let header: Vec<Line> = vec![
            Line::from(vec!["/// ".gray(), self.description.to_string().gray()]),
            Line::from(self.name.to_string()),
        ];
        let variants: Vec<Line> = self
            .variants
            .iter()
            .map(|variant| {
                Line::from(vec![
                    variant.name.to_span(),
                    " /// ".gray(),
                    variant.note.to_span().gray(),
                ])
            })
            .collect();
        let footer = Line::from("\n".to_string());

        let text: Vec<Line> = header
            .into_iter()
            .chain(variants.into_iter())
            .chain(iter::once(footer))
            .collect();

        ListItem::new(Text::from(text))
    }

    pub fn add_variant(&mut self) {
        self.variants.push(Variant::new());
    }
}
