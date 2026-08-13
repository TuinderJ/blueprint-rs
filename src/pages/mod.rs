use crate::{Action, AppData, AppState};
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
    Structs,
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

    pub fn render(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        match self {
            Page::None => {}
            Page::Home => home_page::render(area, buf, state),
            Page::Description { editor } => description_page::render(area, buf, editor),
            Page::Structs => structs_page::render(area, buf),
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
            Page::Structs => todo!(),
            Page::Workflow => todo!(),
        }
    }
}
