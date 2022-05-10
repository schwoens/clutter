use std::fmt::Display;

use chrono::{self, Local, NaiveDate};

#[derive(Clone)]
pub struct Task {
    description: String,
    due_date: NaiveDate,
    completed: bool,
    date_format: String,
}

impl Task {

    pub fn from_string(string: &str, date_format: String) -> Result<Self, String> {

        // get completed
        let split= match string.split_once("] ") {
            Some(s) => s,
            None => return Err("Syntax error in tasks.txt".to_string()),
        };
        let completed = split.0.contains("x"); 

        // get due-date
        let split = match split.1.split_once(": ") {
            Some(s) => s,
            None => return Err("Syntax error in tasks.txt".to_string()),
        };
        let due_date = match NaiveDate::parse_from_str(split.0, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => return Err(format!("Error while parsing task: {}", e)),
        };

        let description = split.1.to_string();
        Ok(Self{description, due_date, completed, date_format})
    }

    pub fn is_overdue(&self) -> bool {
        self.due_date < Local::now().date().naive_local()
    }

    pub fn is_today(&self) -> bool {
        self.due_date == Local::now().date().naive_local()
    }

    pub fn is_future(&self) -> bool {
        self.due_date > Local::now().date().naive_local()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl Display for Task {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut due_date_string = self.due_date.format(&self.date_format).to_string();
        if self.due_date == Local::now().date().naive_local() {
            due_date_string = "today".to_string();
        } else if self.due_date == Local::now().date().naive_local().succ() {
            due_date_string = "tomorrow".to_string();
        } else if self.due_date == Local::now().date().naive_local().pred() {
            due_date_string = "yesterday".to_string();
        }

        let mut cross = "";
        if self.completed {
            cross = "✓ ";
        }

        write!(f, "{}{}: {}", cross, due_date_string, self.description)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.due_date == other.due_date
    }

}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.due_date.cmp(&other.due_date))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due_date.cmp(&other.due_date)
    }
}