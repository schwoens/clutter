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
                    for i in 1..args.len() {
                        match args[i].as_str() {
                            "--show_completed" | "-c" => show_completed = true,
                            "--today" | "-t" => only_today = true,
                            _ => (),
                        }
                    }
                    self.list(show_completed, only_today)
                },
                "e" | "edit" => self.edit(),
                _ => Err("Invalid argument".to_string())
            }
        } else {
            self.list(false, false)
        }
    }

    pub fn list(&mut self, show_completed: bool, only_today: bool) -> Result<(), String> {
        self.task_handler.load_tasks()?;

        if show_completed {
            self.print_all(self.task_handler.get_completed(only_today), self.config.completed_color);
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
                    Err(e) => return Err(format!("Error while trying to edit tasks.txt: {}", e)),
                }
        }
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





