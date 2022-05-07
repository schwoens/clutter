use std::fs;
use crate::task::Task;

pub struct TaskHandler {
    tasks: Vec<Task>,
    datadir: String,
}

impl TaskHandler {

    pub fn new(datadir: String) -> Result<Self, String> {
        Ok(Self{tasks: vec![], datadir})
    }

    pub fn load_tasks(&mut self) -> Result<(), String> {
        self.tasks = self.create_task_vec()?;
        Ok(())
    }

    fn read_tasks(&self) -> Result<String, String> {
        let mut path = self.datadir.clone();
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

    pub fn get_overdue(&self) -> Vec<Task> {
        self.tasks.clone()
            .into_iter()
            .filter(|t| t.is_overdue() && !t.is_completed())
            .collect()
    }

    pub fn get_today(&self) -> Vec<Task> {
        self.tasks.clone()
            .into_iter()
            .filter(|t| t.is_today() && !t.is_completed())
            .collect()
    }

    pub fn get_scheduled(&self) -> Vec<Task> {
        self.tasks.clone()
            .into_iter()
            .filter(|t| t.is_future() && !t.is_completed())
            .collect()
    }
}
