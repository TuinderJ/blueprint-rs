use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use ratatui_textarea::TextArea;

use crate::{Action, AppState, pages::PageKind};

pub fn render(area: Rect, buf: &mut Buffer, editor: &TextArea) {
    let outer_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let title = Line::from(" Blueprint Manager ".bold());
    let legend = Line::from(vec![
        "Quit Without Saving ".into(),
        "<Esc>".blue().bold(),
        " Submit Changes ".into(),
        "<Ctrl + S>".blue().bold(),
    ]);

    let outer_block = Block::new()
        .title(title.centered())
        .title_bottom(legend.centered());

    let middle_area = outer_block.inner(outer_area).inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    outer_block.render(outer_area, buf);

    let middle_block = Block::bordered()
        .border_set(border::THICK)
        .title(Line::from(" Description ".bold()).centered());
    let inner_area = middle_block.inner(middle_area);
    middle_block.render(middle_area, buf);

    editor.render(inner_area, buf)
}

pub fn handle_key_event(
    key_event: KeyEvent,
    state: &mut AppState,
    editor: &mut TextArea<'static>,
) -> Action {
    match key_event.code {
        KeyCode::Esc => Action::GoToPage(PageKind::Home),
        KeyCode::Char('s') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                state.data.description = editor.lines().join("\n");
                return Action::GoToPage(PageKind::Home);
            }
            editor.input(key_event);
            Action::None
        }
        _ => {
            editor.input(key_event);
            Action::None
        }
    }
}
