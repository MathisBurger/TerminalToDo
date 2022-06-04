use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde_json::Result;
use platform_dirs::{AppDirs, UserDirs};
use serde::{Serialize, Deserialize};

pub struct StorageHandler {
    root_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub finished: bool,
    pub title: String,
    pub group: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct DataFile {
    pub tasks: Vec<Task>,
    pub groups: Vec<String>
}


impl StorageHandler {

    pub fn new() -> Self {
        let dirs = AppDirs::new(Some("terminalToDo"), false).unwrap();
        StorageHandler {
            root_dir: dirs.data_dir
        }
    }

    fn read_tasks_file(&mut self) -> String {
        let path = PathBuf::as_path(&self.root_dir);
        let str_path = path.to_str().unwrap().to_owned() + "/data.json";
        let raw_file = File::open(&str_path);
        let initial_data = r#"{"tasks": [], "groups": []}"#;
        let mut file = match raw_file {
            Ok(f) => f,
            Err(e) => {
                let mut f = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(str_path)
                    .unwrap();
                f.write_all(initial_data.as_ref()).unwrap();
                f
            }
        };
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed reading config");
        if data.is_empty() {
            return String::from(initial_data);
        }
        data
    }

    fn get_data(&mut self) -> DataFile {
        let read_data = self.read_tasks_file();
        println!("{}", read_data);
        let content: DataFile = serde_json::from_str(read_data.as_str()).unwrap();
        content
    }

    fn write_data(&mut self, data: DataFile) {
        let raw = serde_json::to_string(&data).unwrap();
        let path = PathBuf::as_path(&self.root_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_str().unwrap().to_owned() + "/data.json")
            .unwrap();
        file.write_all(raw.as_str().as_ref()).expect("Cannot write data");

    }

    pub fn get_all_tasks(&mut self) -> Vec<Task> {
        let content = self.get_data();
        content.tasks
    }

    pub fn add_single_task(&mut self, title: String) {
        let mut data = self.get_data();
        data.tasks.push(Task {
            finished: false,
            title,
            group: None
        });
        self.write_data(data);
    }

}