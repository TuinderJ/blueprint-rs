use crate::{
    Action, AppData, AppState, ArgumentBox, FieldBox, MethodBox, VariantBox,
    data_types::{Enum, Struct, Trait},
};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui_textarea::TextArea;
use strum::{Display, EnumIter};

mod description_page;
mod enums_page;
mod home_page;
mod structs_page;
mod traits_page;
mod workflow_page;

pub const DESCRIPTION_INDEX: usize = 0;
pub const STRUCTS_INDEX: usize = 1;
pub const ENUMS_INDEX: usize = 2;
pub const TRAITS_INDEX: usize = 3;

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
    Traits {
        description_box: TextArea<'static>,
        name_box: TextArea<'static>,
        method_boxes: Vec<MethodBox>,
    },
}

pub enum PageKind {
    Home,
    Description,
    Structs,
    Enums,
    Traits,
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

        let name_text = if current_enum.name.eq("New Enum") {
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

    pub fn traits(state: &AppState) -> Self {
        let empty_trait = Trait::default();
        let current_trait = state
            .data
            .traits
            .get(state.traits_list_state.selected().unwrap_or_default())
            .unwrap_or(&empty_trait);

        let method_boxes = if current_trait.methods.len() == 0 {
            let argument_boxes = vec![ArgumentBox {
                name_box: TextArea::from(vec!["".to_string()]),
                argument_type_box: TextArea::from(vec!["".to_string()]),
            }];
            vec![MethodBox {
                name_box: TextArea::from(vec!["".to_string()]),
                return_type_box: TextArea::from(vec!["".to_string()]),
                argument_boxes,
            }]
        } else {
            current_trait
                .methods
                .iter()
                .map(|method| {
                    let argument_boxes = method
                        .arguments
                        .iter()
                        .map(|argument| ArgumentBox {
                            name_box: TextArea::from(vec![argument.name.to_string()]),
                            argument_type_box: TextArea::from(vec![
                                argument.argument_type.to_string(),
                            ]),
                        })
                        .collect();
                    MethodBox {
                        name_box: TextArea::from(vec![method.name.to_string()]),
                        return_type_box: TextArea::from(vec![method.return_type.to_string()]),
                        argument_boxes,
                    }
                })
                .collect()
        };

        let name_text = if current_trait.name.eq("New Trait") {
            "".to_string()
        } else {
            current_trait.name.to_string()
        };
        Page::Traits {
            description_box: TextArea::from(vec![current_trait.description.to_string()]),
            name_box: TextArea::from(vec![name_text]),
            method_boxes: method_boxes,
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
            Page::Traits {
                description_box,
                name_box,
                method_boxes,
            } => traits_page::render(area, buf, state, description_box, name_box, method_boxes),
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
            Page::Traits {
                description_box,
                name_box,
                method_boxes,
            } => traits_page::handle_key_event(
                key_event,
                state,
                description_box,
                name_box,
                method_boxes,
            ),
        }
    }
}
