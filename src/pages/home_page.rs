use crate::{
    Action, AppState, DESCRIPTION_INDEX, STRUCTS_INDEX, WORKFLOW_INDEX,
    pages::{Page, PageKind},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, StatefulWidget, Widget},
};
use strum::IntoEnumIterator;

pub fn render(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    let outer_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let title = Line::from(" Blueprint Manager ".bold());
    let legend = Line::from(vec![
        "Navigate ".into(),
        "<↓/↑> or <j/k>".blue().bold(),
        " Select ".into(),
        "<Enter>".blue().bold(),
        " Quit ".into(),
        "<Q>".blue().bold(),
        " Submit Changes ".into(),
        "<Ctrl + S>".blue().bold(),
    ]);

    let outer_block = Block::new()
        .title(title.centered())
        .title_bottom(legend.centered());

    let inner_area = outer_block.inner(outer_area);

    outer_block.render(outer_area, buf);

    let [left_pane, right_pane] =
        Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
            .margin(1)
            .areas(inner_area);

    // Left Pane
    let left_block = Block::bordered().border_set(border::THICK);
    let items: Vec<ListItem> = Page::iter()
        .map(|item| ListItem::new(item.to_string()))
        .collect();
    let list = List::new(items)
        .block(left_block)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    StatefulWidget::render(list, left_pane, buf, &mut state.home_list_state);

    // Right Pane
    let right_block = Block::bordered()
        .title(Line::from(" Preview ".bold()))
        .border_set(border::THICK);

    // Determine what items should be rendered on the prewiew panel based on which item is selected.
    let mut items: Vec<ListItem> = vec![];
    if state.home_list_state.selected() == Some(DESCRIPTION_INDEX) {
        items = state
            .data
            .description
            .split("\n")
            .map(|item| ListItem::from(item.to_string()))
            .collect();
    } else if state.home_list_state.selected() == Some(STRUCTS_INDEX) {
        items = state
            .data
            .structs
            .iter()
            .map(|item| item.to_list_item())
            .collect();
    }
    let list = List::new(items).block(right_block);

    Widget::render(list, right_pane, buf);
}

pub fn handle_key_event(key_event: KeyEvent, state: &mut AppState) -> Action {
    match key_event.code {
        KeyCode::Char('q') => return Action::Exit,
        KeyCode::Char('j') | KeyCode::Down => state.home_list_state.select_next(),
        KeyCode::Char('k') | KeyCode::Up => state.home_list_state.select_previous(),
        KeyCode::Enter => {
            if state.home_list_state.selected() == Some(DESCRIPTION_INDEX) {
                return Action::GoToPage(PageKind::Description);
            } else if state.home_list_state.selected() == Some(STRUCTS_INDEX) {
                return Action::GoToPage(PageKind::Structs);
            } else if state.home_list_state.selected() == Some(WORKFLOW_INDEX) {
                return Action::GoToPage(PageKind::Workflow);
            };
        }
        KeyCode::Char('s') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                return Action::GenerateBlueprint;
            }
        }
        _ => {}
    }
    Action::None
}
