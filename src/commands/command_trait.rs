/// Defines the base information of a command
/// that needs to be delivered
pub struct CommandInfo {
    pub title: String,
    pub description: String,
    pub usage: String,
}

/// Defines methods that need to be implemented
/// by a command to ensure that the command is working properly
pub trait Command {
    fn execute(&mut self);
    fn get_command_info(&mut self) -> CommandInfo;
}
