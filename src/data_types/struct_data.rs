use std::iter;

use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Struct {
    description: String,
    name: String,
    fields: Vec<Field>,
}

impl Struct {
    pub fn new() -> Self {
        Self {
            description: "New Struct".to_string(),
            name: "TestStruct".to_string(),
            fields: vec![
                Field {
                    name: "Field 1".to_string(),
                    field_type: "String".to_string(),
                    note: "This is for field 1".to_string(),
                },
                Field {
                    name: "Field 2".to_string(),
                    field_type: "String".to_string(),
                    note: "This is another string".to_string(),
                },
            ],
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
struct Field {
    name: String,
    field_type: String,
    note: String,
}
