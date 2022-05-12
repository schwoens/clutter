use std::{fs::{self, OpenOptions}, io::Write};
use chrono::NaiveDate;

use crate::task::Task;

pub struct TaskHandler {
    tasks: Vec<Task>,
    datadir: String,
    date_format: String,
}

impl TaskHandler {

    pub fn new(datadir: String, date_format: String) -> Result<Self, String> {
        Ok(Self{tasks: vec![], datadir, date_format})
    }

    pub fn load_tasks(&mut self) -> Result<(), String> {
        self.tasks = self.create_task_vec()?;
        Ok(())
    }

    pub fn add_task(&mut self, due_date: &str, description: &str) -> Result<(), String> {
        let path = self.get_or_create_path()?;

        let mut task_string = String::from("[ ] ");
        task_string.push_str(&self.parse_date(due_date)?);
        task_string.push_str(": ");
        task_string.push_str(description);
        task_string.push_str("\n");

        match Task::from_string(&task_string, self.date_format.clone()) {
            Ok(_) => (),
            Err(_) => return Err(format!("Invalid argument"))
        }

        let mut file = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(path) {
                Ok(f) => f,
                Err(e) => return Err(format!("Error while opening tasks.txt: {}", e)),
            };
        match file.write_all(task_string.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(e) => Err(format!("Error while writing tasks.txt: {}", e)),
        }
    }

    fn parse_date(&self, date: &str) -> Result<String, String> {
       let d = match date {
            "today" => chrono::Local::today().naive_local().to_string(),
            "yesterday" => chrono::Local::today().naive_local().pred().to_string(),
            "tomorrow" => chrono::Local::today().naive_local().succ().to_string(),
            _ => date.to_string(),
        };

        match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
            Ok(_) => (),
            Err(_) => return Err(format!("Invalid due date")),
        }
        Ok(d)
    }

    fn read_tasks(&self) -> Result<String, String> {
        let path = self.get_or_create_path()?;
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
                match Task::from_string(str, self.date_format.clone()) {
                    Ok(t) => task_vec.push(t),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(task_vec)
    }

    pub fn get_overdue(&self) -> Vec<Task> {
        let mut overdue = self.tasks.clone();
        overdue.sort();
        overdue.into_iter()
            .filter(|t| t.is_overdue() && !t.is_completed())
            .collect()
    }

    pub fn get_today(&self) -> Vec<Task> {
        let mut today = self.tasks.clone();
        today.sort();
        today.into_iter()
            .filter(|t| t.is_today() && !t.is_completed())
            .collect()
    }

    pub fn get_scheduled(&self) -> Vec<Task> {
        let mut scheduled = self.tasks.clone();
        scheduled.sort();
        scheduled.into_iter()
            .filter(|t| t.is_future() && !t.is_completed())
            .collect()
    }

    pub fn get_completed(&self, only_today: bool) -> Vec<Task> {
        let mut completed =self.tasks.clone();

        completed.sort();

        if only_today {
            completed.into_iter()
                .filter(|t| t.is_completed() && t.is_today())
                .collect()
            
        } else {
            completed.into_iter()
                .filter(|t| t.is_completed())
                .collect()
        }
    }

    pub fn get_or_create_path(&self) -> Result<String, String> {

        let mut path = self.datadir.clone();
        path.push_str("tasks.txt");

        if !std::path::Path::new(&path).exists() {
            match fs::File::create(&path) {
                Ok(_) => (),
                Err(e) => return Err(format!("Error while creating tasks.txt: {}", e)),
            }
        }
        Ok(path)
    }
}
