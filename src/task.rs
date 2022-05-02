use std::fmt::Display;

use chrono::{self, Local, NaiveDate};

pub struct Task {
    description: String,
    due_date: NaiveDate,
    completed: bool,
}

impl Task {

    pub fn from_string(string: &str) -> Option<Self> {

        // get completed
        let split= match string.split_once(" ") {
            Some(s) => s,
            None => return None,
        };
        let completed = split.0.contains("x"); 

        // get due-date
        let split = match split.1.split_once(": ") {
            Some(s) => s,
            None => return None,
        };
        let due_date = match NaiveDate::parse_from_str(split.0, "%y %m %d") {
            Ok(d) => d,
            Err(_) => return None,
        };

        let description = split.1.to_string();

        Some(Self{description, due_date, completed})

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
}

impl Display for Task {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.due_date.to_string(), self.description)
    }
}