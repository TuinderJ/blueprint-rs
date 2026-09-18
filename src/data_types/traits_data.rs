use std::iter;

use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Trait {
    pub description: String,
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub argument_type: String,
}

impl Trait {
    pub fn new() -> Self {
        Self {
            description: "".to_string(),
            name: "New Trait".to_string(),
            methods: vec![Method::default()],
        }
    }

    pub fn to_list_item(&self) -> ListItem<'_> {
        let header: Vec<Line> = vec![
            Line::from(vec!["/// ".gray(), self.description.to_string().gray()]),
            Line::from(vec!["trait: ".to_span(), self.name.to_span()]),
            Line::default(),
        ];

        let mut methods: Vec<Line> = vec![];
        for method in &self.methods {
            methods.push(Line::from(vec![
                method.name.to_span(),
                " => ".gray(),
                method.return_type.to_span(),
                "{".to_span(),
            ]));

            for argument in &method.arguments {
                methods.push(Line::from(vec![
                    "    ".to_span(),
                    argument.name.to_span(),
                    ": ".to_span(),
                    argument.argument_type.to_span(),
                ]));
            }
            methods.push(Line::from("}"));
            methods.push(Line::from(""));
        }
        let footer = vec![Line::from("".to_string()), Line::from("".to_string())];

        let text: Vec<Line> = header
            .into_iter()
            .chain(methods.into_iter())
            .chain(footer.into_iter())
            .collect();

        ListItem::new(Text::from(text))
    }

    pub fn add_method(&mut self) {
        self.methods.push(Method::new());
    }
}

impl Method {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            return_type: "".to_string(),
            arguments: vec![Argument::default()],
        }
    }

    pub fn add_argument(&mut self) {
        // self.arguments.push(Argument::default());
        self.arguments.push(Argument {
            name: "".to_string(),
            argument_type: "".to_string(),
        });
    }
}
