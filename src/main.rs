use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, StatefulWidget},
};
use ratatui_textarea::TextArea;

mod pages;
use crate::{data_types::AppData, pages::PageKind};
use pages::Page;
mod data_types;

const JSON_FILE: &str = ".blueprint.json";
const MD_FILE: &str = "blueprint.md";

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
    mode: Mode,
    active_input: ActiveInput,
    home_list_state: ListState,
    structs_list_state: ListState,
    enums_list_state: ListState,
    workflow_list_state: ListState,
}

enum Action {
    None,
    GoToPage(PageKind),
    UpdatePreview,
    Exit,
    GenerateBlueprint,
}

#[derive(PartialEq, Default, Clone, Copy)]
enum Mode {
    #[default]
    Display,
    Edit,
}

impl Mode {
    pub fn toggle(&mut self) {
        *self = match *self {
            Mode::Display => Mode::Edit,
            Mode::Edit => Mode::Display,
        }
    }
}

#[derive(Default)]
enum ActiveInput {
    #[default]
    None,
    Description,
    Name,
    Field(usize, usize),
    Variant(usize, usize),
}

#[derive(Debug, Clone)]
pub struct FieldBox {
    pub field_name_box: TextArea<'static>,
    pub field_type_box: TextArea<'static>,
    pub field_note_box: TextArea<'static>,
}

#[derive(Debug, Clone)]
pub struct VariantBox {
    pub name_box: TextArea<'static>,
    pub note_box: TextArea<'static>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let cwd = std::env::current_dir().expect("Failed to get current working directory.");
        let src_dir = get_project_root(&cwd)
            .unwrap_or_else(create_new_project)
            .join("src");

        let file_json = src_dir.clone().join(JSON_FILE);
        // TODO: check if the markdown file has changed to alert the user
        let _file_md = src_dir.join(MD_FILE);

        let exists = file_json.try_exists();
        let mut state = match exists {
            std::result::Result::Ok(true) => AppState::from(file_json),
            std::result::Result::Ok(false) => AppState::new(),
            std::result::Result::Err(_) => AppState::new(),
        };

        //Ok(_) => AppState::from(file_json),
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
            Action::UpdatePreview => state.reload_page(),
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
        state.structs_list_state.select(Some(0));
        state.enums_list_state.select(Some(0));
        state.workflow_list_state.select(Some(0));
        state
    }

    // TODO: better error handling
    fn from(path: PathBuf) -> Self {
        let mut state = Self::new();
        let json = fs::read_to_string(path).expect("Failed to read from json file");
        state.data = serde_json::from_str(&json).expect("Failed to parse json");
        state
    }

    fn go_to_page(&mut self, page: PageKind) {
        match page {
            PageKind::Home => self.page = Page::home(),
            PageKind::Description => self.page = Page::description(&self.data),
            PageKind::Structs => self.page = Page::structs(&self),
            PageKind::Enums => self.page = Page::enums(&self),
            PageKind::Workflow => todo!(),
        }
    }

    fn reload_page(&mut self) {
        match &self.page {
            Page::None | Page::Description { editor: _ } | Page::Home => {}
            Page::Structs {
                description_box: _,
                name_box: _,
                field_boxes: _,
            } => self.page = Page::structs(&self),
            Page::Enums {
                description_box: _,
                name_box: _,
                variant_boxes: _,
            } => self.page = Page::enums(&self),
            Page::Workflow => todo!(),
        }
    }

    pub fn set_active_input(&mut self, new_input: ActiveInput) {
        self.active_input = new_input;
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }

    fn generate_blueprint(&mut self) {
        let cwd = std::env::current_dir().expect("Failed to get current working directory.");
        let src_dir = get_project_root(&cwd)
            .unwrap_or_else(create_new_project)
            .join("src");

        let output_file_json = src_dir.clone().join(JSON_FILE);
        let output_file_md = src_dir.join(MD_FILE);

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
