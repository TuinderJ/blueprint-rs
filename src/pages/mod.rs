use crate::{Action, AppData, AppState, FieldBox, data_types::Struct};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui_textarea::TextArea;
use strum::{Display, EnumIter};

pub mod description_page;
pub mod home_page;
mod structs_page;
mod workflow_page;

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
    Workflow,
}

pub enum PageKind {
    Home,
    Description,
    Structs,
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
            Page::Workflow => Action::None,
        }
    }
}
