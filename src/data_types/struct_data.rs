use std::iter;

use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Struct {
    pub description: String,
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new() -> Self {
        Self {
            description: "".to_string(),
            name: "New Struct".to_string(),
            fields: vec![Field {
                name: "".to_string(),
                field_type: "".to_string(),
                note: "".to_string(),
            }],
        }
    }

    pub fn empty() -> Self {
        Self {
            description: "".to_string(),
            name: "".to_string(),
            fields: vec![],
        }
    }

    pub fn to_list_item(&self) -> ListItem<'_> {
        let header: Vec<Line> = vec![
            Line::from(vec!["/// ".gray(), self.description.to_string().gray()]),
            Line::from(self.name.to_string()),
        ];
        let fields: Vec<Line> = self
            .fields
            .iter()
            .map(|field| {
                Line::from(vec![
                    field.name.to_span(),
                    " | ".to_span(),
                    field.field_type.to_span(),
                    " /// ".gray(),
                    field.note.to_span().gray(),
                ])
            })
            .collect();
        let footer = Line::from("\n".to_string());

        let text: Vec<Line> = header
            .into_iter()
            .chain(fields.into_iter())
            .chain(iter::once(footer))
            .collect();

        ListItem::new(Text::from(text))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub note: String,
}
