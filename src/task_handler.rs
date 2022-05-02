use std::{path::Path, fs};
use crate::task::Task;
use dirs;

pub struct TaskHandler {
    config: Config,
}

impl TaskHandler {

    pub fn init() -> Result<Self, String> {

        let config = match Config::load() {
            Ok(c) => c,
            Err(s) => return Err(s), 
        };

        Ok(Self{config})
    }

    pub fn edit(&self) {
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        match edit::edit_file(Path::new(&path)){
            Err(_) => std::process::exit(0),
            Ok(()) => (),
        }
    }

    pub fn complete(&self, index: &str) {
        todo!();
    }

    pub fn list(&self) {
        let task_vec = match self.read_tasks(){
            Some(v) => v,
            None => return,
        };
        for i in 0..task_vec.len() {
            println!("{} {}", i, task_vec[i]);
        }
    }

    fn read_tasks(&self) -> Option<Vec<Task>> {
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        let task_string = match fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => return None,
        };
        self.create_task_vec(task_string.unwrap().split("\n").collect())
    }

    fn create_task_vec(&self, strs: Vec<&str>) -> Option<Vec<Task>> {
        let mut task_vec = vec![];
        for str in strs {
            match Task::from_string(str) {
                Some(t) => task_vec.push(t),
                None => return None,
            }
        }
        Some(task_vec)
    }

    fn get_overdue(&self, task_vec: Vec<Task>) -> Vec<Task> {
        task_vec.into_iter()
            .filter(|t| t.is_overdue())
            .collect()
    }

    fn get_today(&self, task_vec: Vec<Task>) -> Vec<Task> {
        task_vec.into_iter()
            .filter(|t| t.is_today())
            .collect()
    }

    fn get_scheduled(&self, task_vec: Vec<Task>) -> Vec<Task> {
        task_vec.into_iter()
            .filter(|t| t.is_future())
            .collect()
    }
}

struct Config {
    datadir: String,
}

impl Config {
    fn load() -> Result<Self, String> {

        let cdir_path = match dirs::config_dir() {
            Some(p) => p,
            None => return Err("Could not find config directory".to_string()),
        };
        let mut cfile_path = cdir_path.to_str()
            .unwrap()
            .to_string();
        cfile_path.push_str("/clutter/");

        // check if clutter config directory exists and create one if it doesn't
        let mut metadata = match fs::metadata(&cfile_path) {
            Ok(m) => Some(m),
            Err(_) => None,
        };
        
        if metadata.is_some() {
            if !metadata.unwrap().is_dir() {
                match fs::create_dir(Path::new(&cfile_path)) {
                    Ok(_) => (),
                    Err(e) => return Err(format!("Error while creating config directory: {}", e).to_string()),
                }
            }
        } else {
            match fs::create_dir(Path::new(&cfile_path)) {
                Ok(_) => (),
                Err(e) => return Err(format!("Error while creating config directory: {}", e).to_string()),
            }
        }


        //check if clutter config file exists
        cfile_path.push_str("/clutter.conf");
        metadata = match fs::metadata(&cfile_path) {
            Ok(m) => Some(m),
            Err(_) => None,
        };

        if metadata.is_some() {
            if metadata.unwrap().is_file() {
                match Self::read_config(&cfile_path) {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
        }
        match fs::File::create(&cfile_path) {
            Ok(_) => (),
            Err(e) => return Err(format!("Error while creating config file: {}", e).to_string()),
        }
        match Self::read_config(&cfile_path) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_config(path: &str) -> Result<Self, String> {

        // default data directory
        let mut datadir = dirs::config_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        datadir.push_str("/clutter/tasks.txt");

        let config_string = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while reading clutter.conf: {}", e).to_string()),
        };
        for config_line in config_string.lines() {
            let config = match config_line.split_once("=") {
                Some(s) => s,
                None => return Err("Syntax error in clutter.conf".to_string()),
            };

            match config.0 {
                "datadir" => datadir = config.1.to_string(),
                _ => return Err("Syntax error in clutter.conf".to_string()),
            }
        }
        Ok(Self{datadir})
    }
}