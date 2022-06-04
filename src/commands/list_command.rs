use std::error::Error;
use dialoguer::console::Term;
use dialoguer::{MultiSelect, Select};
use dialoguer::theme::ColorfulTheme;
use crate::commands::command_trait::{Command, CommandInfo};
use crate::inputs::error_handling::InputErrorHandling;
use crate::storage_handler::{StorageHandler, Task};

pub struct ListCommand {
    title: String,
    description: String,
    usage: String,
    storage_handler: StorageHandler
}

impl InputErrorHandling for ListCommand {
    fn handle_select_error(&mut self, input: std::io::Result<Option<usize>>) -> Option<usize> {
        match input {
            Err(e) => {
                println!("ERROR: {}", e.description());
                None
            }
            Ok(val) => val
        }
    }
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            title: String::from("List"),
            description: String::from("Lists all tasks that are not done yet"),
            usage: String::from("Just type list"),
            storage_handler: StorageHandler::new()
        }
    }

    fn get_checked_symbol(&mut self, task: &Task) -> String {
        if task.finished {
            return "✅".to_string();
        }
        return "❌".to_string();
    }
}

impl Command for ListCommand {

    fn execute(&mut self) {

        let mut single_tasks = self.storage_handler.get_all_tasks()
           .into_iter()
           .filter(|task| task.group.is_none())
           .collect::<Vec<Task>>();

        let items = (&single_tasks)
            .into_iter()
            .map(|x| self.get_checked_symbol(&x) + "  " +  &x.title)
            .collect::<Vec<String>>();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match self.handle_select_error(selection) {
            None => println!("An error occurred while selecting"),
            Some(val) => {
                let mut actual = (single_tasks[val]).clone();
                actual.finished = !actual.finished;
                single_tasks[val] = actual;
                self.storage_handler.write_task_data(single_tasks);
            }
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