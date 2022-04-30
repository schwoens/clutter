use std::{io::{Stdout, Result}, path::Path, fs};
use term::{Terminal, color};
use crate::task::Task;
use dirs;

pub struct TaskHandler {
    terminal: Box<dyn Terminal<Output = Stdout> + Send>,
    config: Config,
}

impl TaskHandler {


    pub fn init(terminal: Box<dyn Terminal<Output = Stdout> + Send>) -> Self {

        let config = Config::load();

        Self{terminal, config}
    }

    pub fn handle_args(&mut self, args: Vec<String>) {
        if !args.is_empty() {
            match args[0].as_str() {
                "edit" | "e" => self.edit(),
                "complete" | "c" => {
                    if args.len() > 1 {
                        self.complete(&args[1]);
                    }
                },
                "list" | "l" => self.list(),
                _ => {
                    self.println("Invalid arguments", Some(color::BRIGHT_RED))
                },
            }
            return;
        }
        self.list();
    }

    fn edit(&mut self) {
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        match edit::edit_file(Path::new(&path)){
            Err(_) => self.println("Error editing tasks", Some(color::BRIGHT_RED)),
            Ok(()) => (),
        }
    }

    fn complete(&self, index: &str) {
        todo!();
    }

    fn list(&mut self) {
        let mut path = self.config.datadir.clone();
        path.push_str("tasks.txt");
        let tasks = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                self.print("Could not find tasks file. Use ", Some(color::BRIGHT_RED));
                self.print("clutter edit ", Some(color::BRIGHT_YELLOW));
                self.println("to create one.", Some(color::BRIGHT_RED));
                return;
            }
        };

        let task_vec = self.create_task_vec(tasks.split("\n"));
        todo!();
    }

    fn create_task_vec(&self, tasks: core::str::Split<&str>) -> Vec<Task> {
        for task in tasks {
        }
        todo!();
    }

    fn get_overdue(&self, task_vec: Vec<Task>) -> Vec<Task> {
        todo!();
    }

    fn get_today(&self, task_vec: Vec<Task>) -> Vec<Task> {
        todo!();
    }

    fn get_scheduled(&self, task_vec: Vec<Task>) -> Vec<Task> {
        todo!();
    }

    fn get_unscheduled(&self, task_vec: Vec<Task>) -> Vec<Task> {
        todo!();
    }

    fn println(&mut self, text: &str, color: Option<u32>) {
        if color.is_some() {
            self.terminal.fg(color.unwrap());
        }
        writeln!(self.terminal, "{}", text);
        self.terminal.reset().unwrap();
    }

    fn print(&mut self, text: &str, color: Option<u32>) {
        if color.is_some() {
            self.terminal.fg(color.unwrap());
        }
        write!(self.terminal, "{}", text);
        self.terminal.reset().unwrap();
    }
}

struct Config {
    datadir: String,
}

impl Config {
    fn load() -> Self {

        let cdir_path = dirs::config_dir().unwrap();
        let mut cfile_path = cdir_path.to_str()
            .unwrap()
            .to_string();
        cfile_path.push_str("/clutter/clutter.conf");

        let mut datadir = cdir_path.to_str()
            .unwrap()
            .to_string();
        datadir.push_str("/clutter/tasks.txt");

        let configs = fs::read_to_string(cfile_path);
        for config_line in configs {
            let config = match config_line.split_once("=") {
                Some(s) => s,
                None => {
                    Self::syntax_error();
                    std::process::exit(0);
                },
            };
            
            match config.0 {
                "datadir" => datadir = config.1.to_string(),
                _ => {
                    Self::syntax_error();
                    std::process::exit(0);
                },
            }
        }
        Self{datadir}
    }

    fn syntax_error() {
        let mut t = term::stdout().unwrap();
        t.fg(color::BRIGHT_RED);
        writeln!(t, "Syntax error in clutter.conf");
        t.reset().unwrap();
    }
}