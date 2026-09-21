use ratatui::{
    style::Stylize,
    text::{Line, Text, ToSpan},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

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
        let mut header: Vec<Line> = vec![];
        if !self.description.is_empty() {
            header.push(Line::from(vec![
                "/// ".dark_gray(),
                self.description.to_string().dark_gray(),
            ]))
        };
        header.push(Line::from(vec![
            "trait ".magenta(),
            self.name.to_span(),
            " {".to_span(),
        ]));

        let mut methods: Vec<Line> = vec![];
        for method in &self.methods {
            methods.push(Line::from(vec![
                "    fn ".to_span(),
                method.name.to_span().cyan(),
                " (".to_span(),
            ]));

            for argument in &method.arguments {
                let argument_type = if argument.argument_type.is_empty() {
                    "unknown...".dark_gray()
                } else {
                    argument.argument_type.to_span()
                };
                methods.push(Line::from(vec![
                    "        ".to_span(),
                    argument.name.to_span().yellow(),
                    ": ".to_span(),
                    argument_type,
                ]));
            }
            methods.push(Line::from(if method.return_type.is_empty() {
                vec!["    );".to_span()]
            } else {
                vec![
                    "    ) -> ".to_span(),
                    method.return_type.to_span(),
                    ";".to_span(),
                ]
            }));
            methods.push(Line::default());
        }
        methods.push(Line::from("}".to_string()));

        let footer = vec![Line::from("".to_string()), Line::from("".to_string())];

        let text: Vec<Line> = header.into_iter().chain(methods).chain(footer).collect();

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
