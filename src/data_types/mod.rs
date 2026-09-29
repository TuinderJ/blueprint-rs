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

    pub fn to_markdown(&self) -> String {
        // Header
        let mut markdown: String = format!("# Description\n{}\n\n", self.description.to_string());

        // Structs
        if self.structs.len() != 0 {
            markdown += "# Structs\n```rust\n";
            self.structs.iter().for_each(|current| {
                // Description
                if !current.description.is_empty() {
                    markdown = format!("{}/// {}\n", markdown, current.description);
                }

                // Name
                markdown = format!("{}struct {} {{\n", markdown, current.name);

                // Fields
                current.fields.iter().for_each(|field| {
                    // Field notes
                    if !field.note.is_empty() {
                        markdown = format!("{}\t/// {}\n", markdown, field.note);
                    }
                    // Field name and type
                    if field.field_type.is_empty() {
                        markdown = format!("{}\t{}: unknown,\n", markdown, field.name);
                    } else {
                        markdown = format!("{}\t{}: {},\n", markdown, field.name, field.field_type);
                    };
                });

                // Footer
                markdown = format!("{}}}\n\n", markdown);
            });
            markdown += "```\n\n";
        }

        // Enums
        if self.enums.len() != 0 {
            markdown += "# Enums\n```rust\n";
            self.enums.iter().for_each(|current| {
                // Description
                if !current.description.is_empty() {
                    markdown = format!("{}/// {}\n", markdown, current.description);
                }

                // Name
                markdown = format!("{}enum {} {{\n", markdown, current.name);

                // Variants
                current.variants.iter().for_each(|variant| {
                    // Field notes
                    if !variant.note.is_empty() {
                        markdown = format!("{}\t/// {}\n", markdown, variant.note);
                    }
                    markdown = format!("{}\t{},\n", markdown, variant.name);
                });

                // Footer
                markdown = format!("{}}}\n\n", markdown);
            });
            markdown += "```\n\n";
        }

        // Traits
        if self.traits.len() != 0 {
            markdown += "# Traits\n```rust\n";
            self.traits.iter().for_each(|current| {
                // Description
                if !current.description.is_empty() {
                    markdown = format!("{}/// {}\n", markdown, current.description);
                }

                // Name
                markdown = format!("{}trait {} {{\n", markdown, current.name);

                // Methods
                current.methods.iter().for_each(|method| {
                    if method.arguments.len() > 0 {
                        markdown = format!("{}\tfn {} (\n", markdown, method.name);
                        method.arguments.iter().for_each(|argument| {
                            if argument.argument_type.is_empty() {
                                markdown = format!("{}\t\t{}: unknown,\n", markdown, argument.name);
                            } else {
                                markdown = format!(
                                    "{}\t\t{}: {},\n",
                                    markdown, argument.name, argument.argument_type
                                );
                            }
                        });
                        if method.return_type.is_empty() {
                            markdown = format!("{}\t);\n", markdown);
                        } else {
                            markdown = format!("{}\t) -> {};\n", markdown, method.return_type);
                        };

                        markdown += "\n";
                    } else {
                        if method.return_type.is_empty() {
                            markdown = format!("{}\tfn {} ();\n", markdown, method.name);
                        } else {
                            markdown = format!(
                                "{}\tfn {} () -> {};\n",
                                markdown, method.name, method.return_type
                            );
                        }

                        markdown += "\n";
                    };
                });

                // Footer
                markdown = format!("{}}}\n\n", markdown);
            });
            markdown += "```\n\n";
        }

        // CLI Sub Commands
        if self.cli_commands.len() != 0 {
            markdown += "# CLI Sub-Commands\n```bash\n";
            self.cli_commands.iter().for_each(|command| {
                if command.description.is_empty() {
                    markdown = format!("{}{}\n\n", markdown, command.name);
                } else {
                    markdown = format!(
                        "{}{}\n# {}\n\n",
                        markdown, command.name, command.description
                    );
                }
            });
            markdown += "```\n\n";
        }

        markdown
    }
}
