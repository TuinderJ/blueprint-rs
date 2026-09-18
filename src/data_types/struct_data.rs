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
            fields: vec![],
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
        let mut header: Vec<Line> = vec![];
        if !self.description.is_empty() {
            header.push(Line::from(vec![
                "/// ".dark_gray(),
                self.description.to_string().dark_gray(),
            ]));
        }
        header.push(Line::from(vec![
            "struct ".magenta(),
            self.name.to_span(),
            " {".to_span(),
        ]));

        let mut fields: Vec<Line> = vec![];
        for field in &self.fields {
            if !field.note.is_empty() {
                fields.push(Line::from(vec![
                    "    /// ".dark_gray(),
                    field.note.to_span().dark_gray(),
                ]));
            }

            let field_type = if field.field_type.is_empty() {
                "unknown...".dark_gray()
            } else {
                field.field_type.to_span()
            };

            fields.push(Line::from(vec![
                "    ".to_span(),
                field.name.to_span().yellow(),
                ": ".to_span(),
                field_type,
                ",".to_span(),
            ]))
        }
        fields.push(Line::from("}".to_string()));

        let footer = Line::from("\n".to_string());

        let text: Vec<Line> = header
            .into_iter()
            .chain(fields.into_iter())
            .chain(iter::once(footer))
            .collect();

        ListItem::new(Text::from(text))
    }

    pub fn add_field(&mut self) {
        self.fields.push(Field::new());
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub note: String,
}

impl Field {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            field_type: "".to_string(),
            note: "".to_string(),
        }
    }
}
