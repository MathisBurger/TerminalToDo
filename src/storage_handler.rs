use std::fs;
use platform_dirs::{AppDirs, UserDirs};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Defines the base structure of
/// the storage handler
pub struct StorageHandler {
    root_dir: PathBuf,
}

/// A Task tyoe that defines how a task looks in the memory
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub finished: bool,
    pub title: String,
    pub group: Option<String>,
}

/// The base structure of the json file that contains all
/// the important data about the cli.
#[derive(Serialize, Deserialize)]
pub struct DataFile {
    pub tasks: Vec<Task>,
    pub groups: Vec<String>,
}

impl StorageHandler {
    /// Creates a new instance of the storage service
    /// and returns it.
    pub fn new() -> Self {
        let dirs = AppDirs::new(Some("terminalToDo"), false).unwrap();
        StorageHandler {
            root_dir: dirs.data_dir,
        }
    }

    /// Reads the raw string from the data.json file
    /// Furthermore, the file is created and the default data written,
    /// if the file does not exist
    fn read_tasks_file(&mut self) -> String {
        let path = PathBuf::as_path(&self.root_dir);
        if !Path::exists(&path) {
            fs::create_dir(path.clone()).expect("Cannot create root data directory");
        }
        let str_path = path.to_str().unwrap().to_owned() + "/data.json";
        let raw_file = File::open(&str_path);
        let initial_data = r#"{"tasks": [], "groups": []}"#;
        let mut file = match raw_file {
            Ok(f) => f,
            Err(e) => {
                let mut f = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .read(true)
                    .truncate(true)
                    .open(str_path)
                    .unwrap();
                f.write_all(initial_data.as_ref()).unwrap();
                f
            }
        };
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed reading config");
        if data.is_empty() {
            return String::from(initial_data);
        }
        data
    }

    /// Gets the parsed data of the data.json file
    /// as the DataFile struct
    fn get_data(&mut self) -> DataFile {
        let read_data = self.read_tasks_file();
        let content: DataFile = serde_json::from_str(read_data.as_str()).unwrap();
        content
    }

    /// Writes the provided data as string into
    /// the data.json file
    fn write_data(&mut self, data: DataFile) {
        let raw = serde_json::to_string(&data).unwrap();
        let path = PathBuf::as_path(&self.root_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_str().unwrap().to_owned() + "/data.json")
            .unwrap();
        file.write_all(raw.as_str().as_ref())
            .expect("Cannot write data");
    }

    /// Gets all tasks that are provided by the data.json file
    /// and loads them in live time
    pub fn get_all_tasks(&mut self) -> Vec<Task> {
        let content = self.get_data();
        content.tasks
    }

    /// Gets all groups that are provided by the data.json
    /// file. Only the names are returned
    pub fn get_all_groups(&mut self) -> Vec<String> {
        let content = self.get_data();
        content.groups
    }

    /// Adds a single task to the big lists of tasks
    /// and writes them into the data.json file
    pub fn add_single_task(&mut self, title: String, group: Option<String>) {
        let mut data = self.get_data();
        let mut new_id = 0;
        if data.tasks.len() > 0 {
            new_id = data.tasks.last().unwrap().id+1;
        }
        data.tasks.push(Task {
            id: new_id,
            finished: false,
            title,
            group,
        });
        self.write_data(data);
    }

    /// Adds a new task group to the data.json file
    /// and saves it.
    pub fn add_task_group(&mut self, title: String) {
        let mut data = self.get_data();
        data.groups.push(title);
        self.write_data(data);
    }

    /// Checks if an task with specific ID already exists
    /// in vector.
    pub fn id_exists_in_vec(&mut self, data: &Vec<Task>, id: u64) -> bool {
        for i in data.iter() {
            if i.id == id {
                return true;
            }
        }
        return false;
    }

    /// Writes an vector of tasks into the
    /// data.json file
    pub fn write_task_data(&mut self, mut data: Vec<Task>) {
        let mut file_data = self.get_data();
        let mut new_tasks = vec![];
        for el in data {
            if !self.id_exists_in_vec(&new_tasks, el.id) {
                new_tasks.push(el);
            }
        }
        for el in file_data.tasks {
            if !self.id_exists_in_vec(&new_tasks, el.id) {
                new_tasks.push(el);
            }
        }
        file_data.tasks = new_tasks;
        self.write_data(file_data);
    }
}
