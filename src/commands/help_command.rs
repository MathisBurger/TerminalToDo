use crate::commands::command_trait::{Command, CommandInfo};
use crate::commands::list_command::ListCommand;
use crate::handler::commands::Commands;
use crate::handler::commands::Commands::Help;

pub struct HelpCommand {
    title: String,
    description: String,
    usage: String
}

impl HelpCommand {
    pub fn new() -> HelpCommand {
        HelpCommand {
            title: String::from("Help"),
            description: String::from("Shows information about all commands"),
            usage: String::from("Just type help"),
        }
    }
}

impl Command for HelpCommand {

    fn execute(&mut self) {
        let commands = vec![
            HelpCommand::new().get_command_info(),
            ListCommand::new().get_command_info(),
        ];
        for command in commands {
            println!("TITLE: {}", command.title);
            println!("DESCRIPTION: {}", command.description);
            println!("USAGE: {}", command.usage);
            print!("\n\n");
        }
    }

    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone()
        }
    }
}