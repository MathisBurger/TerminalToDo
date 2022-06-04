use crate::commands::command_trait::{Command, CommandInfo};

pub struct ListCommand {
    title: String,
    description: String,
    usage: String
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            title: String::from("List"),
            description: String::from("Lists all tasks that are not done yet"),
            usage: String::from("Just type list")
        }
    }
}

impl Command for ListCommand {

    fn execute(&mut self) {
        println!("List command")
    }

    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone()
        }
    }
}