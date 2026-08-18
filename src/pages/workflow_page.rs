use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::AppState;

pub fn render(_area: Rect, _buf: &mut Buffer) {
    todo!()
}

pub fn _handle_key_event(key_event: KeyEvent, _state: &mut AppState) {
    match key_event.code {
        _ => {}
    }
}
