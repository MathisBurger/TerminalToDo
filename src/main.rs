use crate::commands::command_trait::Command;
use crate::handler::command_handler::CommandHandler;

mod commands;
mod handler;
mod inputs;
mod storage_handler;

fn main() {
    let mut command_handler = CommandHandler::new(std::env::args());
    let command = command_handler.get_command();
    match command {
        Some(cmd) => command_handler.execute_by_enum(cmd),
        None => commands::help_command::HelpCommand::new().execute()
    }
}
