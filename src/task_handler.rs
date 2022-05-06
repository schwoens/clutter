use std::{path::Path, fs, process, io::Write};
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

    pub fn edit(&self) -> Result<(), String>{
        let mut path = self.config.datadir.as_str().to_string();
        path.push_str("tasks.txt");

        if self.config.editor.as_str() == "" {
            // open in default editor
            match edit::edit_file(Path::new(&path)){
                Err(e) => return Err(format!("Error while trying to edit tasks.txt: {}", e)),
                Ok(()) => Ok(()),
            }
        } else {
            // open in prefered editor
            match process::Command::new(&self.config.editor)
                .arg(path) 
                .stdin(process::Stdio::inherit())
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit())
                .output() {
                    Ok(_) => Ok(()),
                    Err(e) => return Err(format!("Error while trying to edit task.txt: {}", e)),
                }
        }
    }

    pub fn complete(&self, index: &str) -> Result<(), String> {
        let i = match index.parse::<usize>() {
            Ok(i) => i,
            Err(e) => return Err(format!("Error while trying to complete task: {}", e)),
        };
        let mut task_string = "[ ] ".to_string();
        task_string.push_str(&(self.create_task_vec()?[i].to_string()));

        let mut tasks: String = self.read_tasks()?
            .lines()
            .filter(|s| s != &task_string)
            .collect();
        task_string = task_string.replace("[ ]", "[x]");
        tasks.push_str(&task_string);

        // write tasks.txt
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        let mut file = match fs::File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error while opening tasks.txt: {}", e)),
        };
        match file.write_all(tasks.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error while writing tasks.txt: {}", e)),
        }
    }

    pub fn list(&self) -> Result<String, String> {
        let task_vec = self.create_task_vec()?;

        let mut output = "".to_string();
        for i in 0..task_vec.len() {
            output.push_str(&format!("{} {}\n", i, task_vec[i]))
        }
        Ok(output)
    }

    fn read_tasks(&self) -> Result<String, String> {
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        match fs::read_to_string(path) {
            Ok(s) => Ok(s),
            Err(e) => return Err(format!("Error while reading tasks.txt: {}", e)),
        }
    }

    fn create_task_vec(&self) -> Result<Vec<Task>, String> {
        
        let task_string = self.read_tasks()?;
        let mut task_vec = vec![];
        for str in task_string.lines() {
            if str != "" {
                match Task::from_string(str) {
                    Ok(t) => task_vec.push(t),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(task_vec)
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
    editor: String,
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

        // defaults
        let mut datadir = dirs::config_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        datadir.push_str("/clutter/");

        let mut editor = "".to_string();

        

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
                "editor" => editor = config.1.to_string(),
                _ => return Err("Syntax error in clutter.conf".to_string()),
            }
        }
        Ok(Self{datadir, editor})
    }
}