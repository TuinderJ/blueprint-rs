use crate::{
    Action, AppData, AppState, FieldBox, VariantBox,
    data_types::{Enum, Struct},
};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui_textarea::TextArea;
use strum::{Display, EnumIter};

pub mod description_page;
mod enums_page;
pub mod home_page;
mod structs_page;
mod workflow_page;

pub const DESCRIPTION_INDEX: usize = 0;
pub const STRUCTS_INDEX: usize = 1;
pub const ENUMS_INDEX: usize = 2;
pub const WORKFLOW_INDEX: usize = 3;

#[derive(Clone, Display, Debug, Default, EnumIter)]
pub enum Page {
    #[strum(disabled)]
    None,
    #[default]
    #[strum(disabled)]
    Home,
    Description {
        editor: TextArea<'static>,
    },
    Structs {
        description_box: TextArea<'static>,
        name_box: TextArea<'static>,
        field_boxes: Vec<FieldBox>,
    },
    Enums {
        description_box: TextArea<'static>,
        name_box: TextArea<'static>,
        variant_boxes: Vec<VariantBox>,
    },
    Workflow,
}

pub enum PageKind {
    Home,
    Description,
    Structs,
    Enums,
    Workflow,
}

impl Page {
    pub fn home() -> Self {
        Page::Home
    }

    pub fn description(data: &AppData) -> Self {
        Page::Description {
            editor: TextArea::from(data.description.split("\n")),
        }
    }

    pub fn structs(state: &AppState) -> Self {
        let empty_struct = Struct::empty();
        let current_struct = state
            .data
            .structs
            .get(state.structs_list_state.selected().unwrap_or_default())
            .unwrap_or(&empty_struct);

        let field_boxes = if current_struct.fields.len() == 0 {
            vec![FieldBox {
                field_name_box: TextArea::from(vec!["".to_string()]),
                field_type_box: TextArea::from(vec!["".to_string()]),
                field_note_box: TextArea::from(vec!["".to_string()]),
            }]
        } else {
            current_struct
                .fields
                .iter()
                .map(|field| FieldBox {
                    field_name_box: TextArea::from(vec![field.name.to_string()]),
                    field_type_box: TextArea::from(vec![field.field_type.to_string()]),
                    field_note_box: TextArea::from(vec![field.note.to_string()]),
                })
                .collect()
        };

        let name_text = if current_struct.name.eq("New Struct") {
            "".to_string()
        } else {
            current_struct.name.to_string()
        };
        Page::Structs {
            description_box: TextArea::from(vec![current_struct.description.to_string()]),
            name_box: TextArea::from(vec![name_text]),
            field_boxes: field_boxes,
        }
    }

    pub fn enums(state: &AppState) -> Self {
        let empty_enum = Enum::empty();
        let current_enum = state
            .data
            .enums
            .get(state.enums_list_state.selected().unwrap_or_default())
            .unwrap_or(&empty_enum);

        let variant_boxes = if current_enum.variants.len() == 0 {
            vec![VariantBox {
                name_box: TextArea::from(vec!["".to_string()]),
                note_box: TextArea::from(vec!["".to_string()]),
            }]
        } else {
            current_enum
                .variants
                .iter()
                .map(|variant| VariantBox {
                    name_box: TextArea::from(vec![variant.name.to_string()]),
                    note_box: TextArea::from(vec![variant.note.to_string()]),
                })
                .collect()
        };

        let name_text = if current_enum.name.eq("New Struct") {
            "".to_string()
        } else {
            current_enum.name.to_string()
        };
        Page::Enums {
            description_box: TextArea::from(vec![current_enum.description.to_string()]),
            name_box: TextArea::from(vec![name_text]),
            variant_boxes: variant_boxes,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        match self {
            Page::None => {}
            Page::Home => home_page::render(area, buf, state),
            Page::Description { editor } => description_page::render(area, buf, editor),
            Page::Structs {
                description_box,
                name_box,
                field_boxes,
            } => structs_page::render(area, buf, state, description_box, name_box, field_boxes),
            Page::Enums {
                description_box,
                name_box,
                variant_boxes,
            } => enums_page::render(area, buf, state, description_box, name_box, variant_boxes),
            Page::Workflow => workflow_page::render(area, buf),
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut AppState) -> Action {
        match self {
            Page::None => Action::None,
            Page::Home => home_page::handle_key_event(key_event, state),
            Page::Description { editor } => {
                description_page::handle_key_event(key_event, state, editor)
            }
            Page::Structs {
                description_box,
                name_box,
                field_boxes,
            } => structs_page::handle_key_event(
                key_event,
                state,
                description_box,
                name_box,
                field_boxes,
            ),
            Page::Enums {
                description_box,
                name_box,
                variant_boxes,
            } => enums_page::handle_key_event(
                key_event,
                state,
                description_box,
                name_box,
                variant_boxes,
            ),
            Page::Workflow => Action::None,
        }
    }
}
