use crossterm::event::{
    KeyCode::{self},
    KeyEvent, KeyModifiers,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{Action, ActiveInput, AppState, CommandBox, data_types::Command, pages::PageKind};

fn next_active_input(state: &mut AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Command(command_index, sub_index) => {
            if sub_index == 0 {
                ActiveInput::Command(command_index, 1)
            } else {
                if command_index + 1 < state.data.cli_commands.len() {
                    ActiveInput::Command(command_index + 1, 0)
                } else {
                    ActiveInput::Command(command_index, sub_index)
                }
            }
        }
        _ => unreachable!(),
    }
}

fn previous_active_input(state: &AppState) -> ActiveInput {
    match state.active_input {
        ActiveInput::Command(command_index, sub_index) => {
            if sub_index == 1 {
                ActiveInput::Command(command_index, 0)
            } else {
                if command_index == 0 {
                    ActiveInput::Command(command_index, sub_index)
                } else {
                    ActiveInput::Command(command_index - 1, 1)
                }
            }
        }
        _ => unreachable!(),
    }
}

pub fn render(area: Rect, buf: &mut Buffer, state: &mut AppState, command_boxes: &Vec<CommandBox>) {
    if !matches!(state.active_input, ActiveInput::Command(_, _)) {
        state.set_active_input(ActiveInput::Command(0, 0));
    };

    let outer_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let title = Line::from(" Blueprint Manager ".bold());
    let legend = Line::from(vec![
        "Navigate ".into(),
        "<Tab/Shift+Tab>".blue().bold(),
        " New Command ".into(),
        "<CTRL + N>".blue().bold(),
        " Back ".into(),
        "<ESC>".blue().bold(),
    ]);

    let outer_block = Block::new()
        .title(title.centered())
        .title_bottom(legend.centered());

    let inner_area = outer_block.inner(outer_area);
    outer_block.render(outer_area, buf);

    let mut constraints: Vec<Constraint> = vec![];
    for _ in command_boxes {
        constraints.push(Constraint::Length(8));
    }

    let areas = Layout::vertical(constraints).split(inner_area);

    for (index, command_box) in command_boxes.iter().enumerate() {
        let block = Block::bordered();
        let area = block.inner(areas[index]);
        block.render(areas[index], buf);

        let [name_area, description_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

        // Name
        let color = match state.active_input {
            ActiveInput::Command(command_index, row) => {
                if command_index == index && row == 0 {
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
            .title(" Command Name ");
        let block_area = block.inner(name_area);
        block.render(name_area, buf);
        command_box.name_box.render(block_area, buf);

        // Description
        let color = match state.active_input {
            ActiveInput::Command(command_index, row) => {
                if command_index == index && row == 1 {
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
            .title(" Description / Usage ");
        let block_area = block.inner(description_area);
        block.render(description_area, buf);
        command_box.description_box.render(block_area, buf);
    }
}

pub fn handle_key_event(
    key_event: KeyEvent,
    state: &mut AppState,
    command_boxes: &mut [CommandBox],
) -> Action {
    match key_event.code {
        KeyCode::Esc => {
            state.data.cli_commands = command_boxes
                .iter()
                .filter_map(|command_box| {
                    let name = command_box.name_box.lines()[0].to_string();
                    if name.is_empty() {
                        return None;
                    }
                    let description = command_box.description_box.lines()[0].to_string();
                    Some(Command { name, description })
                })
                .collect();
            state.set_active_input(ActiveInput::None);
            Action::GoToPage(PageKind::Home)
        }
        KeyCode::Tab => {
            update_commands(state, command_boxes);
            state.active_input = next_active_input(state);
            Action::None
        }
        KeyCode::BackTab => {
            update_commands(state, command_boxes);
            state.active_input = previous_active_input(state);
            Action::None
        }
        KeyCode::Enter => Action::None,
        KeyCode::Char('n') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                update_commands(state, command_boxes);
                state.data.add_command();
                Action::UpdatePreview
            } else {
                update_inputs(key_event, state, command_boxes);
                Action::None
            }
        }
        _ => {
            update_inputs(key_event, state, command_boxes);
            Action::None
        }
    }
}

fn update_commands(state: &mut AppState, command_boxes: &mut [CommandBox]) {
    state.data.cli_commands = command_boxes
        .iter()
        .map(|command_box| {
            let name = command_box.name_box.lines()[0].to_string();
            let description = command_box.description_box.lines()[0].to_string();
            Command { name, description }
        })
        .collect();
}

fn update_inputs(key_event: KeyEvent, state: &AppState, command_boxes: &mut [CommandBox]) {
    match state.active_input {
        ActiveInput::Command(command_index, sub_index) => {
            if sub_index == 0 {
                command_boxes[command_index].name_box.input(key_event);
            } else {
                command_boxes[command_index]
                    .description_box
                    .input(key_event);
            }
        }
        _ => unreachable!(),
    }
}
