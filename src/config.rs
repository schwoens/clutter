use std::{fs, path::Path};

pub struct Config {
    pub datadir: String,
    pub editor: String,
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