use std::{fs, path::Path};

use term::{color::Color, color};

pub struct Config {
    pub datadir: String,
    pub editor: String,
    pub date_format: String,
    pub overdue_color: Color,
    pub today_color: Color,
    pub scheduled_color: Color,
    pub completed_color: Color,
}

impl Config {
    pub fn load() -> Result<Self, String> {

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

        let mut date_format = "%Y-%m-%d".to_string();

        let mut overdue_color = color::RED;
        let mut today_color = color::YELLOW;
        let mut scheduled_color = color::CYAN;
        let mut completed_color = color::GREEN;
        
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
                "date_format" => date_format = config.1.to_string(),
                "overdue_color" => overdue_color = Self::match_color(config.1)?,
                "today_color" => today_color = Self::match_color(config.1)?,
                "scheduled_color" => scheduled_color = Self::match_color(config.1)?,
                "completed_color" => completed_color = Self::match_color(config.1)?,
                _ => return Err("Syntax error in clutter.conf".to_string()),
            }
        }
        Ok(Self{datadir, editor, date_format, overdue_color, today_color, scheduled_color, completed_color})
    }

    fn match_color(string: &str) -> Result<u32, String> {
        match string.to_lowercase().as_str() {
            "black" => Ok(color::BLACK),
            "blue" => Ok(color::BLUE),
            "bright_black" => Ok(color::BRIGHT_BLACK),
            "bright_blue" => Ok(color::BRIGHT_BLUE),
            "bright_cyan" => Ok(color::BRIGHT_CYAN),
            "bright_green" => Ok(color::BRIGHT_GREEN),
            "bright_magenta" => Ok(color::BRIGHT_MAGENTA),
            "bright_red" => Ok(color::BRIGHT_RED),
            "bright_white" => Ok(color::BRIGHT_WHITE),
            "bright_yellow" => Ok(color::BRIGHT_YELLOW),
            "cyan" => Ok(color::CYAN),
            "green" => Ok(color::GREEN),
            "magenta" => Ok(color::MAGENTA),
            "red" => Ok(color::RED),
            "white" => Ok(color::WHITE),
            "yellow" => Ok(color::YELLOW),
            _ => Err("Invalid color in clutter.conf".to_string())
        }
    }
}