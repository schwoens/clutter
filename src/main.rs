use std::{path::Path, process};

use task::Task;
use term::color::{self, Color};

use crate::{task_handler::TaskHandler, config::Config};

mod task_handler;
mod task;
mod config;
mod test;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let mut clutter = match Clutter::init() {
        Ok(c) => c,
        Err(e) => {
            Clutter::print_error(e);
            std::process::exit(0);
        },
    };
    
    match clutter.handle_args(args) {
        Ok(_) => (),
        Err(e) => Clutter::print_error(e),
    };
}

struct Clutter {
    config: Config,
    task_handler: TaskHandler,
}

impl Clutter {

    fn init() -> Result<Self, String> {
        let config = Config::load()?;
        let task_handler = TaskHandler::new(config.datadir.clone(), config.date_format.clone())?;
        Ok(Self{config, task_handler})
    }

    fn handle_args(&mut self, args: Vec<String>) -> Result<(), String> {
        if args.len() > 1 {
            match args[1].as_str() {
                "l" | "list" => {
                    let mut show_completed = false;
                    let mut only_today = false;
                    for i in 2..args.len() {
                        match args[i].as_str() {
                            "--show_completed" | "-c" => show_completed = true,
                            "--today" | "-t" => only_today = true,
                            _ => (),
                        }
                    }
                    self.list(show_completed, only_today)
                },
                "e" | "edit" => self.edit(),
                "a" | "add" => {
                    if args.len() < 3 {
                        return Err("Missing argument".to_string());
                    }
                    self.add(args[2].clone())
                },
                _ => Err("Invalid argument".to_string())
            }
        } else {
            self.list(false, false)
        }
    }

    pub fn list(&mut self, show_completed: bool, only_today: bool) -> Result<(), String> {
        self.task_handler.load_tasks()?;

        let uncompleted_tasks_exist = !(self.task_handler.get_overdue().is_empty() && 
            self.task_handler.get_scheduled().is_empty() && 
            self.task_handler.get_today().is_empty());

        let completed_tasks_exist = !self.task_handler.get_completed(only_today).is_empty();

        if show_completed {

            if !completed_tasks_exist {
                Self::print_string("No tasks".to_string(), self.config.notasks_color);
                return Ok(());
            }

            self.print_all(self.task_handler.get_completed(only_today), self.config.completed_color);
        } else {
            
            if !uncompleted_tasks_exist {
                Self::print_string("No tasks".to_string(), self.config.notasks_color);
                return Ok(());
            }
        }

        if !only_today {
            self.print_all(self.task_handler.get_overdue(), self.config.overdue_color);
        }
        self.print_all(self.task_handler.get_today(), self.config.today_color);
        if !only_today {
            self.print_all(self.task_handler.get_scheduled(), self.config.scheduled_color);
        }
        Ok(())
    }

    pub fn add(&mut self, arg: String) -> Result<(), String> {
        let (due_date, description) = match arg.split_once(": ") {
            Some(s) => s,
            None => return Err("Invalid argument".to_string()),
        };
        self.task_handler.add_task(due_date, description)?;
        self.list(false, false)?;
        Ok(())
    }

    pub fn edit(&mut self) -> Result<(), String> {
        let mut path = self.config.datadir.as_str().to_string();
        path.push_str("tasks.txt");

        if self.config.editor.as_str() == "" {
            // open in default editor
            match edit::edit_file(Path::new(&path)){
                Err(e) => return Err(format!("Error while trying to edit tasks.txt: {}", e)),
                Ok(_) => (),
            };
        } else {
            // open in prefered editor
            match process::Command::new(&self.config.editor)
                .arg(path) 
                .stdin(process::Stdio::inherit())
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit())
                .output() {
                    Ok(_) => (),
                    Err(e) => return Err(format!("Error while trying to edit tasks.txt: {}", e)),
                };
        }
        self.list(false, false)?;
        Ok(())
    }


    fn print_all(&self, tasks: Vec<Task>, color: Color) {
        for task in tasks {
            Self::print_string(task.to_string(), color);
        }
    }

    pub fn print_error(message: String) {
        let mut t = term::stdout().unwrap();
        t.fg(color::BRIGHT_RED).unwrap();
        writeln!(t, "{}", message).unwrap();
        t.reset().unwrap();
    }

    pub fn print_string(string: String, color: Color) {
        let mut t = term::stdout().unwrap();
        t.fg(color).unwrap();
        writeln!(t, "{}", string).unwrap();
        t.reset().unwrap();
    }
}





