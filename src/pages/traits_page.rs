use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    Action, ActiveInput, AppState, MethodBox, Mode,
    data_types::{Argument, Method},
    pages::PageKind,
};

fn next_active_input(state: &mut AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Description => ActiveInput::Name,
        ActiveInput::Name => {
            let current_trait = state
                .data
                .traits
                .get_mut(state.traits_list_state.selected().unwrap_or_default())
                .unwrap();
            if current_trait.methods.is_empty() {
                current_trait.add_method();
            }
            ActiveInput::Method(0, 0)
        }
        ActiveInput::Method(method, index) => {
            let current_trait = state
                .data
                .traits
                .get_mut(state.traits_list_state.selected().unwrap_or_default())
                .unwrap();

            let highest_method = current_trait.methods.len() - 1;
            let highest_index = current_trait.methods[method].arguments.len() * 2 + 1;

            let at_highest_method = method == highest_method;
            let at_highest_index = index == highest_index;

            if at_highest_index && at_highest_method {
                return ActiveInput::Method(method, index);
            }
            if at_highest_index {
                return ActiveInput::Method(method + 1, 0);
            }
            ActiveInput::Method(method, index + 1)
        }
        _ => ActiveInput::None,
    }
}

fn previous_active_input(state: &AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Description => ActiveInput::Description,
        ActiveInput::Name => ActiveInput::Description,
        ActiveInput::Method(method, index) => {
            if method == 0 && index == 0 {
                return ActiveInput::Name;
            }
            if index == 0 {
                return ActiveInput::Method(method - 1, 0);
            }
            ActiveInput::Method(method, index - 1)
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
    method_boxes: &[MethodBox],
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
            " New Trait ".into(),
            "<n>".blue().bold(),
            " Back ".into(),
            "<ESC>".blue().bold(),
        ]),
        Mode::Edit => Line::from(vec![
            "Navigate ".into(),
            "<Tab/Shift+Tab>".blue().bold(),
            " New Method ".into(),
            "<Ctrl+n>".blue().bold(),
            " New Argument".into(),
            "<Ctrl+b>".blue().bold(),
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
        .traits
        .iter()
        .map(|item| ListItem::from(item.name.to_string()))
        .collect();

    let list = List::new(list_items)
        .block(left_block)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    StatefulWidget::render(list, left_pane, buf, &mut state.traits_list_state);

    // Right Pane
    let color = match state.mode {
        Mode::Display => Color::DarkGray,
        Mode::Edit => Color::White,
    };
    let right_block = Block::bordered()
        .title(Line::from(" Trait ".bold()))
        .border_set(border::THICK)
        .border_style(Style::default().fg(color));

    let right_outer_area = right_block.inner(right_pane);

    right_block.render(right_pane, buf);

    let [description_area, name_area, methods_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(right_outer_area);

    // Description
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

    // Name
    let color = match state.active_input {
        ActiveInput::Name => Color::White,
        _ => Color::DarkGray,
    };
    let name_block = Block::bordered()
        .border_set(border::THICK)
        .border_style(Style::default().fg(color))
        .title(" Trait Name ");
    let name_block_area = name_block.inner(name_area);
    name_block.render(name_area, buf);
    Widget::render(name_box, name_block_area, buf);

    // Methods
    let method_block = Block::default().title("Methods");
    let method_block_area = method_block.inner(methods_area);
    let mut row_constraints = vec![];
    for method_box in method_boxes {
        row_constraints.push(Constraint::Length(
            method_box.argument_boxes.len() as u16 * 3 + 5,
        ));
    }
    let areas = Layout::vertical(row_constraints).split(method_block_area);
    method_block.render(methods_area, buf);

    for (method_index, method_box) in method_boxes.iter().enumerate() {
        let color = match state.active_input {
            ActiveInput::Method(method, _) => {
                if method == method_index {
                    Color::White
                } else {
                    Color::DarkGray
                }
            }
            _ => Color::DarkGray,
        };
        let outer_block = Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::default().fg(color))
            .title(format!(" {} ", method_box.name_box.lines()[0]));
        let block_area = outer_block.inner(areas[method_index]);
        outer_block.render(areas[method_index], buf);

        let mut row_constraints = vec![Constraint::Length(3)];
        for _ in &method_box.argument_boxes {
            row_constraints.push(Constraint::Length(3));
        }
        let method_areas = Layout::vertical(row_constraints).split(block_area);

        let [name_area, type_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(method_areas[0]);

        let color = match state.active_input {
            ActiveInput::Method(method, index) => {
                if method == method_index && index == 0 {
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
            .title(" Method Name ");
        let block_area = block.inner(name_area);
        block.render(name_area, buf);
        method_box.name_box.render(block_area, buf);

        let color = match state.active_input {
            ActiveInput::Method(method, index) => {
                if method == method_index && index == 1 {
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
            .title(" Return Type ");
        let block_area = block.inner(type_area);
        block.render(type_area, buf);
        method_box.return_type_box.render(block_area, buf);

        // Arguments
        for (index, argument_box) in method_box.argument_boxes.iter().enumerate() {
            let [name_area, type_area] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                    .areas(method_areas[index + 1]);

            let color = match state.active_input {
                ActiveInput::Method(method, argument_index) => {
                    if method == method_index && argument_index == index * 2 + 2 {
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
                .title(" Argument Name ");
            let block_area = block.inner(name_area);
            block.render(name_area, buf);
            argument_box.name_box.render(block_area, buf);

            let color = match state.active_input {
                ActiveInput::Method(method, argument_index) => {
                    if method == method_index && argument_index == index * 2 + 3 {
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
                .title(" Type ");
            let block_area = block.inner(type_area);
            block.render(type_area, buf);
            argument_box.argument_type_box.render(block_area, buf);
        }
    }
    // Widget::render(
    //     Paragraph::new(format!("{:#?}", state.traits_list_state))
    //         .block(Block::bordered().title(" Debug ")),
    //     right_pane,
    //     buf,
    // );
}

pub fn handle_key_event(
    key_event: KeyEvent,
    state: &mut AppState,
    description_box: &mut TextArea,
    name_box: &mut TextArea,
    method_boxes: &mut [MethodBox],
) -> Action {
    match state.mode {
        Mode::Display => match key_event.code {
            KeyCode::Esc => {
                state.data.traits.retain_mut(|item| {
                    item.methods
                        .retain(|method| !method.name.is_empty() || !method.return_type.is_empty());
                    !item.name.is_empty() && item.name != "New trait"
                });
                Action::GoToPage(PageKind::Home)
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if state.traits_list_state.selected().unwrap_or_default()
                    < state.data.traits.len() - 1
                {
                    state.traits_list_state.select_next();
                }
                Action::UpdatePreview
            }
            KeyCode::Char('k') | KeyCode::Up => {
                state.traits_list_state.select_previous();
                Action::UpdatePreview
            }
            KeyCode::Char('n') => {
                state.mode.toggle();
                state.set_active_input(ActiveInput::Description);
                state.data.add_trait();
                state.traits_list_state.select_last();
                Action::UpdatePreview
            }
            KeyCode::Enter => {
                if state.data.traits.is_empty() {
                    return Action::None;
                }
                state.mode.toggle();
                state.set_active_input(ActiveInput::Description);
                Action::UpdatePreview
            }
            _ => Action::None,
        },
        Mode::Edit => match key_event.code {
            KeyCode::Esc => {
                state.mode.toggle();
                Action::UpdatePreview
            }
            // Add a method
            KeyCode::Char('n') => {
                if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    update_active_input(state, key_event, description_box, name_box, method_boxes);
                    return Action::None;
                }
                state
                    .data
                    .traits
                    .get_mut(state.traits_list_state.selected().unwrap_or_default())
                    .unwrap()
                    .add_method();
                Action::UpdatePreview
            }
            // Add an argument to the currently selected method
            KeyCode::Char('b') => {
                if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    update_active_input(state, key_event, description_box, name_box, method_boxes);
                    return Action::None;
                }
                let selected_method = match state.active_input {
                    ActiveInput::Method(method, _) => method,
                    _ => 0,
                };
                state
                    .data
                    .traits
                    .get_mut(state.traits_list_state.selected().unwrap_or_default())
                    .unwrap()
                    .methods
                    .get_mut(selected_method)
                    .unwrap()
                    .add_argument();
                Action::UpdatePreview
            }
            KeyCode::Tab => {
                update_trait(state, description_box, name_box, method_boxes);
                state.active_input = next_active_input(state);
                Action::UpdatePreview
            }
            KeyCode::BackTab => {
                update_trait(state, description_box, name_box, method_boxes);
                state.active_input = previous_active_input(state);
                Action::UpdatePreview
            }
            KeyCode::Enter => {
                state.mode.toggle();
                update_trait(state, description_box, name_box, method_boxes);
                state.set_active_input(ActiveInput::None);
                Action::UpdatePreview
            }
            _ => {
                update_active_input(state, key_event, description_box, name_box, method_boxes);
                Action::None
            }
        },
    }
}

enum DesiredBox {
    None,
    ReturnType {
        method_index: usize,
    },
    Name {
        method_index: usize,
    },
    Argument {
        method_index: usize,
        argument_index: usize,
        argument_box: DesiredArgumentBox,
    },
}

enum DesiredArgumentBox {
    Name,
    Type,
}

fn update_active_input(
    state: &mut AppState,
    key_event: KeyEvent,
    description_box: &mut TextArea,
    name_box: &mut TextArea,
    method_boxes: &mut [MethodBox],
) {
    match state.active_input {
        ActiveInput::Description => {
            description_box.input(key_event);
        }
        ActiveInput::Name => {
            name_box.input(key_event);
        }
        ActiveInput::Method(method, index) => {
            let mut desired_box: DesiredBox = DesiredBox::None;

            for (method_index, method_box) in method_boxes.iter().enumerate() {
                if method == method_index && index == 0 {
                    desired_box = DesiredBox::Name { method_index };
                    break;
                } else if method == method_index && index == 1 {
                    desired_box = DesiredBox::ReturnType { method_index };
                    break;
                }

                for (argument_index, _) in method_box.argument_boxes.iter().enumerate() {
                    if method == method_index && index == argument_index * 2 + 2 {
                        desired_box = DesiredBox::Argument {
                            method_index,
                            argument_index,
                            argument_box: DesiredArgumentBox::Name,
                        };
                        break;
                    } else if method == method_index && index == argument_index * 2 + 3 {
                        desired_box = DesiredBox::Argument {
                            method_index,
                            argument_index,
                            argument_box: DesiredArgumentBox::Type,
                        };
                        break;
                    }
                }
            }

            match desired_box {
                DesiredBox::None => {}
                DesiredBox::ReturnType { method_index } => {
                    let method_box = method_boxes.get_mut(method_index).unwrap();
                    method_box.return_type_box.input(key_event);
                }
                DesiredBox::Name { method_index } => {
                    let method_box = method_boxes.get_mut(method_index).unwrap();
                    method_box.name_box.input(key_event);
                }
                DesiredBox::Argument {
                    method_index,
                    argument_index,
                    argument_box,
                } => {
                    let method_box = method_boxes.get_mut(method_index).unwrap();
                    let argument = method_box.argument_boxes.get_mut(argument_index).unwrap();
                    match argument_box {
                        DesiredArgumentBox::Name => argument.name_box.input(key_event),
                        DesiredArgumentBox::Type => argument.argument_type_box.input(key_event),
                    };
                }
            }
        }
        _ => {}
    };
}

fn update_trait(
    state: &mut AppState,
    description_box: &TextArea,
    name_box: &TextArea,
    method_boxes: &[MethodBox],
) {
    let current_trait = state
        .data
        .traits
        .get_mut(state.traits_list_state.selected().unwrap_or_default())
        .unwrap();

    current_trait.description = description_box.lines()[0].to_string();
    current_trait.name = name_box.lines()[0].to_string();
    current_trait.methods = method_boxes
        .iter()
        .filter_map(|method_box| {
            let name = method_box.name_box.lines()[0].to_string();
            let return_type = method_box.return_type_box.lines()[0].to_string();
            let arguments = method_box
                .argument_boxes
                .iter()
                .map(|argument_box| Argument {
                    name: argument_box.name_box.lines()[0].to_string(),
                    argument_type: argument_box.argument_type_box.lines()[0].to_string(),
                })
                .collect();

            if state.mode == Mode::Display && name.is_empty() && return_type.is_empty() {
                return None;
            }
            Some(Method {
                name,
                return_type,
                arguments,
            })
        })
        .collect();
}
