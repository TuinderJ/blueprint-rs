use std::iter;

use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Enum {
    pub description: String,
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Default, Serialize, Deserialize)]
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
            name: "New Enum".to_string(),
            variants: vec![Variant::new()],
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
        let mut header: Vec<Line> = vec![];

        if !self.description.is_empty() {
            header.push(Line::from(vec![
                "/// ".dark_gray(),
                self.description.to_string().dark_gray(),
            ]));
        }
        header.push(Line::from(vec![
            "enum ".magenta(),
            self.name.to_span(),
            " {".to_span(),
        ]));

        let mut variants: Vec<Line> = vec![];
        for variant in &self.variants {
            if !variant.note.is_empty() {
                variants.push(Line::from(vec![
                    "    /// ".dark_gray(),
                    variant.note.to_span().dark_gray(),
                ]));
            }
            variants.push(Line::from(vec![
                "    ".to_span(),
                variant.name.to_span().yellow(),
                ",".to_span(),
            ]));
        }
        variants.push(Line::from("}".to_string()));

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
