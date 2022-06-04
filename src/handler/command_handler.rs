use std::env::Args;
use crate::Command;
use crate::commands::help_command::HelpCommand;
use crate::commands::list_command::ListCommand;
use crate::handler::commands;
use crate::handler::commands::Commands;

pub struct CommandHandler {
    arguments: Args
}

impl CommandHandler {

    /// Creates a new command handler that can handle
    /// the command line input by default
    pub fn new(args: Args) -> CommandHandler {
        CommandHandler {
            arguments: args
        }
    }

    /// Parses the string of the command into the enum with
    /// all possible commands. If the provided command does not
    /// exist, the help command will be returned
    fn parse_command_to_enum(&mut self, command: &str) -> Option<Commands> {
        match command {
            "help" => Some(Commands::Help),
            "list" => Some(Commands::List),
            _ => None
        }
    }

    /// Gets the command that is passed into the cli and
    /// parses the command into the enum of all existing
    /// commands.
    pub fn get_command(&mut self) -> Option<Commands> {
        match self.arguments.nth(1) {
            Some(command) => self.parse_command_to_enum(command.as_str()),
            None => None,
        }
    }

    /// Executes the provided command by the enum.
    /// All other user inputs are handled by the command
    /// object itself.
    pub fn execute_by_enum(&mut self, cmd: Commands) {
        match cmd {
            Commands::Help => HelpCommand::new().execute(),
            Commands::List => ListCommand::new().execute(),
        }
    }
}