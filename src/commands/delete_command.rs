use dialoguer::console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use serde::de::Unexpected::Str;
use crate::Command;
use crate::commands::command_trait::CommandInfo;
use crate::inputs::error_handling::InputErrorHandling;
use crate::storage_handler::{StorageHandler, Task};

/// Defines the basic structure of the delete command
pub struct DeleteCommand {
    title: String,
    description: String,
    usage: String,
    storage_handler: StorageHandler
}

impl InputErrorHandling for DeleteCommand {}

impl DeleteCommand {

    /// Creates a new instance of the delete command
    /// and initializes the storage handler in it.
    pub fn new() -> DeleteCommand {
        DeleteCommand {
            title: String::from("Delete command"),
            description: String::from("Deletes specific tasks or task groups"),
            usage: String::from("Just type the command"),
            storage_handler: StorageHandler::new()
        }
    }
}

impl Command for DeleteCommand {

    /// This method is called on command execution.
    /// Contains the main code of the command
    fn execute(&mut self) {
        let mut single_tasks = self
            .storage_handler
            .get_all_tasks();

        let items = (&single_tasks)
            .into_iter()
            .map(|x| {
                if x.group.is_some() {
                    return x.title.clone() + " (" + x.group.as_ref().unwrap().as_str() + ")";
                }
                return x.title.clone();
            })
            .collect::<Vec<String>>();

        if items.len() > 0 {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr());

            match self.handle_select_error(selection) {
                None => println!("An error occurred while selecting"),
                Some(val) => {
                    let mut new_values = vec![];
                    for i in 0..single_tasks.len() {
                        if i != val {
                            new_values.push(single_tasks[i].clone());
                        }
                    }
                    self.storage_handler.write_task_data(new_values);
                }
            }
        } else {
            println!("No tasks given");
        }
    }

    /// Gets the basic information about the command
    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone()
        }
    }
}