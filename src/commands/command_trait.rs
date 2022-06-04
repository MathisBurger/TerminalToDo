pub struct CommandInfo {
    pub title: String,
    pub description: String,
    pub usage: String
}

pub trait Command {
    fn execute(&mut self);
    fn get_command_info(&mut self) -> CommandInfo;
}