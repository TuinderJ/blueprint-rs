use std::path::{Path, PathBuf};

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, StatefulWidget},
};
use serde::{Deserialize, Serialize};

mod pages;
use crate::{data_types::Struct, pages::PageKind};
use pages::Page;
mod data_types;

const DESCRIPTION_INDEX: usize = 0;
const STRUCTS_INDEX: usize = 1;
const WORKFLOW_INDEX: usize = 2;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Default)]
struct App {}

#[derive(Default)]
struct AppState {
    should_exit: bool,
    page: Page,
    data: AppData,
    home_list_state: ListState,
    workflow_list_state: ListState,
}

#[derive(Default, Serialize, Deserialize)]
struct AppData {
    description: String,
    structs: Vec<Struct>,
    enums: Vec<Enum>,
    traits: Vec<Trait>,
    // TODO
    module_structure: String,
    error_types: Vec<ErrorType>,
    cli_commands: Vec<Command>,
    external_dependencies: Vec<String>,
    workflow: Workflow,
}

impl AppData {
    fn add_struct(&mut self) {
        self.structs.push(Struct::new());
    }
}

#[derive(Serialize, Deserialize)]
struct Enum {
    description: String,
    name: String,
    variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize)]
struct Variant {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Trait {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorType {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Command {
    name: String,
    description: String,
    subcommands: Vec<Command>,
}

#[derive(Default, Serialize, Deserialize)]
struct Workflow {
    lists: Vec<List>,
}

#[derive(Serialize, Deserialize)]
struct List {
    header: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    completed: bool,
    subtasks: Vec<Task>,
}

enum Action {
    None,
    GoToPage(PageKind),
    Exit,
    GenerateBlueprint,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut state = AppState::new();
        while !state.should_exit {
            terminal.draw(|frame| self.draw(frame, &mut state))?;
            self.handle_events(&mut state)?;
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame, state: &mut AppState) {
        frame.render_stateful_widget(self, frame.area(), state);
    }

    pub fn handle_events(&mut self, state: &mut AppState) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let action = self.handle_key_event(key_event, state);
                self.handle_action(action, state);
            }
            _ => {}
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut AppState) -> Action {
        let mut page = std::mem::replace(&mut state.page, Page::None);
        let action = page.handle_key_event(key_event, state);
        state.page = page;
        action
    }

    pub fn handle_action(&mut self, action: Action, state: &mut AppState) {
        match action {
            Action::None => {}
            Action::GoToPage(page) => state.go_to_page(page),
            Action::Exit => state.exit(),
            Action::GenerateBlueprint => state.generate_blueprint(),
        };
    }
}

impl StatefulWidget for &mut App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let page = std::mem::replace(&mut state.page, Page::None);
        page.render(area, buf, state);
        state.page = page;
    }
}

impl AppState {
    fn new() -> Self {
        let mut state = Self::default();
        state.home_list_state.select(Some(0));
        state.workflow_list_state.select(Some(0));
        state
    }

    pub fn go_to_page(&mut self, page: PageKind) {
        match page {
            PageKind::Home => self.page = Page::home(),
            PageKind::Description => self.page = Page::description(&self.data),
            PageKind::Structs => self.data.add_struct(),
            PageKind::Workflow => todo!(),
        }
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    pub fn generate_blueprint(&mut self) {
        let cwd = std::env::current_dir().expect("Failed to get current working directory.");
        let src_dir = get_project_root(&cwd)
            .unwrap_or_else(create_new_project)
            .join("src");

        let output_file_json = src_dir.clone().join("blueprint.json");
        let output_file_md = src_dir.join("blueprint.md");

        let json_string = serde_json::to_string(&self.data).expect("couldn't parse to json");
        std::fs::write(&output_file_json, json_string).expect("couldn't write to json file");

        let md_string = generate_md_string(&self.data);
        std::fs::write(&output_file_md, md_string).expect("couldn't write to md file");

        self.should_exit = true;
    }
}

fn get_project_root(dir: &Path) -> Option<PathBuf> {
    let mut current = dir.to_path_buf();
    loop {
        if current.join("Cargo.toml").exists() {
            return Some(current);
        };
        if !current.pop() {
            return None;
        }
    }
}

fn create_new_project() -> PathBuf {
    todo!()
}

fn generate_md_string(data: &AppData) -> String {
    data.description.to_string()
}
