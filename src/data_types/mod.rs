mod enum_data;
mod struct_data;

pub use enum_data::*;
use serde::{Deserialize, Serialize};
pub use struct_data::*;

#[derive(Default, Serialize, Deserialize)]
pub struct AppData {
    pub description: String,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub traits: Vec<Trait>,
    // TODO
    pub module_structure: String,
    pub error_types: Vec<ErrorType>,
    pub cli_commands: Vec<Command>,
    pub external_dependencies: Vec<String>,
    pub workflow: Workflow,
}

impl AppData {
    pub fn add_struct(&mut self) {
        self.structs.push(Struct::new());
    }

    pub fn add_enum(&mut self) {
        self.enums.push(Enum::new());
    }
}

#[derive(Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorType {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub subcommands: Vec<Command>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Workflow {
    pub lists: Vec<List>,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    pub header: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub completed: bool,
    pub subtasks: Vec<Task>,
}
