mod commands_data;
mod enum_data;
mod struct_data;
mod traits_data;

pub use commands_data::*;
pub use enum_data::*;
pub use struct_data::*;
pub use traits_data::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct AppData {
    pub description: String,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub traits: Vec<Trait>,
    pub cli_commands: Vec<Command>,
    // TODO: everything after this still needs to be implemented
    pub workflow: Workflow,
}

impl AppData {
    pub fn add_struct(&mut self) {
        self.structs.push(Struct::new());
    }

    pub fn add_enum(&mut self) {
        self.enums.push(Enum::new());
    }

    pub fn add_trait(&mut self) {
        self.traits.push(Trait::new());
    }

    pub fn add_command(&mut self) {
        self.cli_commands.push(Command::default());
    }
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
