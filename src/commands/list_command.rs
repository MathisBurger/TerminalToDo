use crate::commands::command_trait::{Command, CommandInfo};
use crate::inputs::error_handling::InputErrorHandling;
use crate::storage_handler::{StorageHandler, Task};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, MultiSelect, Select};
use std::error::Error;

/// Defines the list command
pub struct ListCommand {
    title: String,
    description: String,
    usage: String,
    storage_handler: StorageHandler,
}

impl InputErrorHandling for ListCommand {}

impl ListCommand {
    /// Creates a new instance of the list command with
    /// the required base data and a new instance of the
    /// storage handler
    pub fn new() -> ListCommand {
        ListCommand {
            title: String::from("List"),
            description: String::from("Lists all tasks that are not done yet"),
            usage: String::from("Just type list"),
            storage_handler: StorageHandler::new(),
        }
    }

    /// Gets the related symbol based on if the task is
    /// finished or not. The returned strings are symbols
    /// and may not be supported by every terminal / cmd
    fn get_checked_symbol(&mut self, task: &Task) -> String {
        if task.finished {
            return "✅".to_string();
        }
        return "❌".to_string();
    }

    /// Opens a submenu with all tasks of a specific group
    /// that can be selected and updated
    fn open_group_prompt(&mut self, title: String) {
        let mut single_tasks = self
            .storage_handler
            .get_all_tasks()
            .into_iter()
            .filter(|task| task.group == Some(title.clone()))
            .collect::<Vec<Task>>();

        let mut items = (&single_tasks)
            .into_iter()
            .map(|x| self.get_checked_symbol(&x) + "  " + &x.title)
            .collect::<Vec<String>>();
        items.push("← back".to_string());
        items = items.into_iter().rev().collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());
        match self.handle_select_error(selection) {
            None => {},
            Some(val) => {
                if val == 0 {
                    return self.execute();
                }
                if !Confirm::new().with_prompt("Save?").interact().unwrap() {
                    return;
                }
                single_tasks = single_tasks.into_iter().rev().collect();
                let mut actual = (single_tasks[val-1]).clone();
                actual.finished = !actual.finished;
                single_tasks[val-1] = actual.clone();
                self.storage_handler.write_task_data(single_tasks);
            }
        }
    }
}

impl Command for ListCommand {
    /// This method is called on command execution.
    /// Contains the main code of the command
    fn execute(&mut self) {
        let mut groups = self.storage_handler.get_all_groups();
        let group_clone = groups.clone();
        let mut single_tasks = self
            .storage_handler
            .get_all_tasks()
            .into_iter()
            .filter(|task| task.group == None)
            .collect::<Vec<Task>>();

        let mut items = (&single_tasks)
            .into_iter()
            .map(|x| self.get_checked_symbol(&x) + "  " + &x.title)
            .collect::<Vec<String>>();
        items.append(&mut groups);

        items = items.into_iter().rev().collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match self.handle_select_error(selection) {
            None => println!("An error occurred while selecting"),
            Some(val) => {
                println!("{}", ((val as i32) - (group_clone.len() as i32)));
                if ((val as i32) - (group_clone.len() as i32)) >= 0 && group_clone.len() != 0 {
                    if !Confirm::new().with_prompt("Save?").interact().unwrap() {
                        return;
                    }
                    let mut actual = (single_tasks[val-group_clone.len()]).clone();
                    actual.finished = !actual.finished;
                    single_tasks[val-group_clone.len()] = actual.clone();
                    self.storage_handler.write_task_data(single_tasks);
                } else {
                    self.open_group_prompt(items[val].clone());
                }
            }
        }
    }

    /// Gets the base command information
    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone(),
        }
    }
}
