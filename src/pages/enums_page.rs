use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, StatefulWidget, Widget},
};
use ratatui_textarea::TextArea;

use crate::{
    Action, ActiveInput, AppState, Mode, VariantBox, data_types::Variant, pages::PageKind,
};

fn next_active_input(state: &mut AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Description => ActiveInput::Name,
        ActiveInput::Name => {
            let current_enum = state
                .data
                .enums
                .get_mut(state.enums_list_state.selected().unwrap_or_default())
                .unwrap();
            if current_enum.variants.len() == 0 {
                current_enum.add_variant();
            }
            ActiveInput::Variant(0, 0)
        }
        ActiveInput::Variant(row, col) => {
            let at_last_col = col == 1;
            let current_enum = state
                .data
                .enums
                .get_mut(state.enums_list_state.selected().unwrap_or_default())
                .unwrap();
            if at_last_col {
                if row == current_enum.variants.len() - 1 {
                    current_enum.add_variant();
                }
                ActiveInput::Variant(row + 1, 0)
            } else {
                ActiveInput::Variant(row, col + 1)
            }
        }
        _ => ActiveInput::None,
    }
}

fn previous_active_input(state: &AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Description => ActiveInput::Description,
        ActiveInput::Name => ActiveInput::Description,
        ActiveInput::Variant(row, col) => {
            let at_first_row = row == 0;
            let at_first_col = col == 0;
            if at_first_row && at_first_col {
                return ActiveInput::Name;
            }
            if at_first_col {
                return ActiveInput::Variant(row - 1, 1);
            }
            ActiveInput::Variant(row, col - 1)
        }
        _ => ActiveInput::None,
    }
}

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    state: &mut AppState,
    description_box: &TextArea,
    name_box: &TextArea,
    variant_boxes: &Vec<VariantBox>,
) {
    let outer_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let title = Line::from(" Blueprint Manager ".bold());
    let legend = match state.mode {
        Mode::Display => Line::from(vec![
            "Navigate ".into(),
            "<↓/↑> or <j/k>".blue().bold(),
            " Select ".into(),
            "<Enter>".blue().bold(),
            " Back ".into(),
            "<ESC>".blue().bold(),
        ]),
        Mode::Edit => Line::from(vec![
            "Navigate ".into(),
            "<TAB/SHIFT + TAB>".blue().bold(),
            " Submit Changes ".into(),
            "<Enter>".blue().bold(),
            " Back ".into(),
            "<ESC>".blue().bold(),
        ]),
    };

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
    let color = match state.mode {
        Mode::Display => Color::White,
        Mode::Edit => Color::DarkGray,
    };
    let left_block = Block::bordered()
        .border_set(border::THICK)
        .border_style(Style::default().fg(color));
    let list_items: Vec<ListItem> = state
        .data
        .enums
        .iter()
        .map(|item| ListItem::from(item.name.to_string()))
        .chain(std::iter::once(ListItem::from(Line::from("+ New Enum"))))
        .collect();

    let list = List::new(list_items)
        .block(left_block)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    StatefulWidget::render(list, left_pane, buf, &mut state.enums_list_state);

    // Right Pane
    let color = match state.mode {
        Mode::Display => Color::DarkGray,
        Mode::Edit => Color::White,
    };
    let right_block = Block::bordered()
        .title(Line::from(" Enum ".bold()))
        .border_set(border::THICK)
        .border_style(Style::default().fg(color));

    let right_outer_area = right_block.inner(right_pane);

    right_block.render(right_pane, buf);

    let [description_area, name_area, variants_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(right_outer_area);

    let color = match state.active_input {
        ActiveInput::Description => Color::White,
        _ => Color::DarkGray,
    };
    let description_block = Block::bordered()
        .border_set(border::THICK)
        .border_style(Style::default().fg(color))
        .title(" Description ");
    let description_block_area = description_block.inner(description_area);
    description_block.render(description_area, buf);
    Widget::render(description_box, description_block_area, buf);

    let color = match state.active_input {
        ActiveInput::Name => Color::White,
        _ => Color::DarkGray,
    };
    let name_block = Block::bordered()
        .border_set(border::THICK)
        .border_style(Style::default().fg(color))
        .title(" Enum Name ");
    let name_block_area = name_block.inner(name_area);
    name_block.render(name_area, buf);
    Widget::render(name_box, name_block_area, buf);

    let variant_block = Block::default().title("Variants");
    let variant_block_area = variant_block.inner(variants_area);
    let areas = Layout::vertical(variant_boxes.iter().map(|_| Constraint::Length(3)))
        .split(variant_block_area);
    variant_block.render(variants_area, buf);

    for (index, variant_box) in variant_boxes.iter().enumerate() {
        let [name_area, note_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(areas[index]);

        let color = match state.active_input {
            ActiveInput::Variant(row, col) => {
                if row == index && col == 0 {
                    Color::White
                } else {
                    Color::DarkGray
                }
            }
            _ => Color::DarkGray,
        };
        let block = Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::default().fg(color))
            .title(" Variant Name ");
        let block_area = block.inner(name_area);
        block.render(name_area, buf);
        variant_box.name_box.render(block_area, buf);

        let color = match state.active_input {
            ActiveInput::Variant(row, col) => {
                if row == index && col == 1 {
                    Color::White
                } else {
                    Color::DarkGray
                }
            }
            _ => Color::DarkGray,
        };
        let block = Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::default().fg(color))
            .title(" Notes ");
        let block_area = block.inner(note_area);
        block.render(note_area, buf);
        variant_box.note_box.render(block_area, buf);
    }
}

pub fn handle_key_event(
    key_event: KeyEvent,
    state: &mut AppState,
    description_box: &mut TextArea,
    name_box: &mut TextArea,
    variant_boxes: &mut Vec<VariantBox>,
) -> Action {
    match key_event.code {
        KeyCode::Esc => match state.mode {
            Mode::Display => {
                state.data.enums.retain_mut(|item| {
                    item.variants
                        .retain(|variant| !variant.name.is_empty() || !variant.note.is_empty());
                    !item.name.is_empty() && !(item.name == "New enum".to_string())
                });
                Action::GoToPage(PageKind::Home)
            }
            Mode::Edit => {
                state.mode.toggle();
                Action::UpdatePreview
            }
        },
        KeyCode::Char('j') | KeyCode::Down => match state.mode {
            Mode::Display => {
                state.enums_list_state.select_next();
                Action::UpdatePreview
            }
            Mode::Edit => {
                update_active_input(state, key_event, description_box, name_box, variant_boxes);
                Action::None
            }
        },
        KeyCode::Char('k') | KeyCode::Up => match state.mode {
            Mode::Display => {
                state.enums_list_state.select_previous();
                Action::UpdatePreview
            }
            Mode::Edit => {
                update_active_input(state, key_event, description_box, name_box, variant_boxes);
                Action::None
            }
        },
        KeyCode::Tab => match state.mode {
            Mode::Display => Action::None,
            Mode::Edit => {
                update_enum(state, description_box, name_box, variant_boxes);
                state.active_input = next_active_input(state);
                Action::UpdatePreview
            }
        },
        KeyCode::BackTab => match state.mode {
            Mode::Display => Action::None,
            Mode::Edit => {
                update_enum(state, description_box, name_box, variant_boxes);
                state.active_input = previous_active_input(state);
                Action::UpdatePreview
            }
        },
        KeyCode::Enter => {
            let should_add_new_enum =
                state.enums_list_state.selected().unwrap_or_default() == state.data.enums.len();

            if should_add_new_enum {
                state.mode.toggle();
                state.set_active_input(ActiveInput::Description);
                state.data.add_enum();
                return Action::UpdatePreview;
            }

            state.mode.toggle();
            update_enum(state, description_box, name_box, variant_boxes);

            let new_input = match state.mode {
                Mode::Display => ActiveInput::None,
                Mode::Edit => ActiveInput::Description,
            };
            state.set_active_input(new_input);
            Action::UpdatePreview
        }
        _ => {
            if state.mode == Mode::Edit {
                update_active_input(state, key_event, description_box, name_box, variant_boxes);
            };
            Action::None
        }
    }
}

fn update_active_input(
    state: &mut AppState,
    key_event: KeyEvent,
    description_box: &mut TextArea,
    name_box: &mut TextArea,
    variant_boxes: &mut Vec<VariantBox>,
) {
    match state.active_input {
        ActiveInput::Description => {
            description_box.input(key_event);
        }
        ActiveInput::Name => {
            name_box.input(key_event);
        }
        ActiveInput::Variant(row, col) => {
            let variant_box = variant_boxes.get_mut(row).unwrap();
            if col == 0 {
                variant_box.name_box.input(key_event);
            } else if col == 1 {
                variant_box.note_box.input(key_event);
            }
        }
        _ => {}
    };
}

fn update_enum(
    state: &mut AppState,
    description_box: &TextArea,
    name_box: &TextArea,
    variant_boxes: &Vec<VariantBox>,
) {
    let current_enum = state
        .data
        .enums
        .get_mut(state.enums_list_state.selected().unwrap_or_default())
        .unwrap();

    current_enum.description = description_box.lines()[0].to_string();
    current_enum.name = name_box.lines()[0].to_string();
    current_enum.variants = variant_boxes
        .iter()
        .filter_map(|variant_box| {
            let name = variant_box.name_box.lines()[0].to_string();
            let note = variant_box.note_box.lines()[0].to_string();

            if state.mode == Mode::Display && name.is_empty() && note.is_empty() {
                return None;
            }
            Some(Variant { name, note })
        })
        .collect();
}
