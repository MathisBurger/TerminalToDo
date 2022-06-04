use crate::commands::command_trait;
use crate::commands::command_trait::CommandInfo;
use crate::inputs::error_handling::InputErrorHandling;
use crate::storage_handler::StorageHandler;
use crate::Command;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use std::error::Error;
use std::io::Result;
use std::ops::{Add, Deref};

#[derive(Copy, Clone)]
enum AddAction {
    SingleTask,
    TaskGroup,
}

/// Defines the whole command that is executed by
/// the command system itself. If the command is executed
/// a new todo entry can be added
pub struct AddCommand {
    title: String,
    description: String,
    usage: String,
    add_action: Option<AddAction>,
    storage_handler: StorageHandler,
}

impl AddCommand {
    /// Creates a new instance of the command with the default
    /// values provided. The storage handler is never reinitialized
    /// and only initialized on Command creation
    pub fn new() -> AddCommand {
        AddCommand {
            title: String::from("Add Command"),
            description: String::from("Makes it possible to add new task groups or single tasks"),
            usage: String::from("just enter the command"),
            add_action: None,
            storage_handler: StorageHandler::new(),
        }
    }

    /// Takes an instance of the AddAction enum and sets
    /// the add_action of the object. Furthermore, the command
    /// will be re-executed after this function call.
    fn submit_add_action_type(&mut self, action: AddAction) {
        self.add_action = Some(action);
        self.execute();
    }

    /// Gets an enum instance by the value that has been selected
    /// in the action input promnt and returns it.
    fn get_add_action_value_by_input(&mut self, value: &str) -> AddAction {
        match value {
            "Add a new task" => AddAction::SingleTask,
            "Add a new task group" => AddAction::TaskGroup,
            _ => AddAction::SingleTask,
        }
    }

    /// Opens an prompt for reading the add_action from the user input.
    /// After that the selected value is saved into the objects memory.
    fn open_add_action_prompt(&mut self) {
        let items = vec!["Add a new task", "Add a new task group"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());
        match self.handle_select_error(selection) {
            None => {}
            Some(u) => {
                let action = self.get_add_action_value_by_input(items[u]);
                self.submit_add_action_type(action);
            }
        }
    }

    /// Opens a prompt for inserting the value for the new
    /// task. If the valze has been submitted a new single
    /// task that is not assigned to any group will be created
    fn add_single_task(&mut self) {
        let input: String = Input::new()
            .with_prompt("Title")
            .interact_text()
            .expect("Failed while inserting data");
        if self.confirm_selection() {
            return self.storage_handler.add_single_task(input);
        }
    }

    /// Opens a prompt for confirming a specific action.
    /// The result of the confirmation will be returned
    fn confirm_selection(&mut self) -> bool {
        Confirm::new().with_prompt("Save?").interact().unwrap()
    }
}

impl InputErrorHandling for AddCommand {
    /// Handles the error of the select menu
    fn handle_select_error(&mut self, input: Result<Option<usize>>) -> Option<usize> {
        match input {
            Err(e) => {
                println!("ERROR: {}", e.description());
                None
            }
            Ok(val) => val,
        }
    }
}

impl Command for AddCommand {
    /// Executes the main code of the command.
    /// This method is called of the command is initialized
    fn execute(&mut self) {
        if self.add_action.is_none() {
            return self.open_add_action_prompt();
        }
        match &self.add_action.as_ref().unwrap() {
            AddAction::SingleTask => self.add_single_task(),
            AddAction::TaskGroup => println!("Not implemented yet"),
        }
    }

    /// Returns the main command infos of the command
    fn get_command_info(&mut self) -> CommandInfo {
        CommandInfo {
            title: self.title.clone(),
            description: self.description.clone(),
            usage: self.usage.clone(),
        }
    }
}
