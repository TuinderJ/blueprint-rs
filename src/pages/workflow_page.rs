use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{Action, AppState};

pub fn render(area: Rect, buf: &mut Buffer) {
    // TODO: workflow page
    todo!()
}

pub fn handle_key_event(key_event: KeyEvent, state: &mut AppState) -> Action {
    // TODO: workflow page
    match key_event.code {
        _ => Action::None,
    }
}
