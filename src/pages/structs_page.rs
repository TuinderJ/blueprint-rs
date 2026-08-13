use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::AppState;

pub fn render(area: Rect, buf: &mut Buffer) {
    todo!()
}

pub fn handle_key_event(key_event: KeyEvent, state: &mut AppState) {
    match key_event.code {
        _ => {}
    }
}
