use std::error::Error;
use std::ops::{Add, Deref};
use dialoguer::console::Term;
use std::io::Result;
use crate::Command;
use crate::commands::command_trait;
use crate::commands::command_trait::CommandInfo;
use dialoguer::{Confirm, Input, Select};
use dialoguer::theme::ColorfulTheme;
use crate::inputs::error_handling::InputErrorHandling;
use crate::storage_handler::StorageHandler;

#[derive(Copy, Clone)]
enum AddAction {
    SingleTask,
    TaskGroup,
}

pub struct AddCommand {
    title: String,
    description: String,
    usage: String,
    add_action: Option<AddAction>,
    storage_handler: StorageHandler
}

impl AddCommand {

    pub fn new() -> AddCommand {
        AddCommand {
            title: String::from("Add Command"),
            description: String::from("Makes it possible to add new task groups or single tasks"),
            usage: String::from("just enter the command"),
            add_action: None,
            storage_handler: StorageHandler::new()
        }
    }

    fn submit_add_action_type(&mut self, action: AddAction) {
        self.add_action = Some(action);
        self.execute();
    }

    fn get_add_action_value_by_input(&mut self, value: &str) -> AddAction {
        match value {
            "Add a new task" => AddAction::SingleTask,
            "Add a new task group" => AddAction::TaskGroup,
            _ => AddAction::SingleTask
        }
    }

    fn open_add_action_prompt(&mut self) {
        let items = vec!["Add a new task", "Add a new task group"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());
        match self.handle_select_error(selection) {
            None => {},
            Some(u) => {
                let action = self.get_add_action_value_by_input(items[u]);
                self.submit_add_action_type(action);
            }
        }
    }

    fn add_single_task(&mut self) {
        let input: String = Input::new()
            .with_prompt("Title")
            .interact_text().expect("Failed while inserting data");
        if self.confirm_selection() {
            return self.storage_handler.add_single_task(input);
        }
        println!("Stopped");
    }

    fn confirm_selection(&mut self) -> bool {
        Confirm::new().with_prompt("Save?").interact().unwrap()
    }
}

impl InputErrorHandling for AddCommand {

    fn handle_select_error(&mut self, input: Result<Option<usize>>) -> Option<usize> {
        match input {
            Err(e) => {
                println!("ERROR: {}", e.description());
                None
            }
            Ok(val) => val
        }
    }
}

impl Command for AddCommand {

    fn execute(&mut self) {
        if self.add_action.is_none() {
            return self.open_add_action_prompt();
        }
        match &self.add_action.as_ref().unwrap() {
            AddAction::SingleTask => self.add_single_task(),
            AddAction::TaskGroup => println!("Not implemented yet")
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