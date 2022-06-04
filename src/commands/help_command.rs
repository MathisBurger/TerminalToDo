use crate::commands::add_command::AddCommand;
use crate::commands::command_trait::{Command, CommandInfo};
use crate::commands::delete_command::DeleteCommand;
use crate::commands::list_command::ListCommand;
use crate::handler::commands::Commands;
use crate::handler::commands::Commands::Help;

/// Defines the base help command
pub struct HelpCommand {
    title: String,
    description: String,
    usage: String,
}

impl HelpCommand {
    /// Creates a new instance of the help command
    /// with the default values.
    pub fn new() -> HelpCommand {
        HelpCommand {
            title: String::from("Help"),
            description: String::from("Shows information about all commands"),
            usage: String::from("Just type help"),
        }
    }
}

impl Command for HelpCommand {
    /// This method is called on command execute.
    /// Contains the main code of the command
    fn execute(&mut self) {
        let commands = vec![
            HelpCommand::new().get_command_info(),
            ListCommand::new().get_command_info(),
            AddCommand::new().get_command_info(),
            DeleteCommand::new().get_command_info()
        ];
        for command in commands {
            println!("TITLE: {}", command.title);
            println!("DESCRIPTION: {}", command.description);
            println!("USAGE: {}", command.usage);
            print!("\n\n");
        }
    }

    /// Returns the base information about the command
    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone(),
        }
    }
}
