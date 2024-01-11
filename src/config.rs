use homedir::get_my_home;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: String,
    working_dir: String,
}

impl Config {
    pub fn new(working_dir: &str) -> Config {
        let path = Config::get_path();
        Config {
            path,
            working_dir: String::from(working_dir),
        }
    }

    pub fn exists() -> bool {
        Path::new(&Config::get_path()).exists()
    }

    pub fn get_path() -> String {
        let mut home_path = get_my_home().unwrap().unwrap().to_path_buf();
        home_path.push(".config/CourseManager/config.toml");
        home_path.to_str().unwrap().to_string()
    }

    pub fn get_working_dir(&self) -> &str {
        &self.working_dir
    }

    pub fn write(&self) {
        let path = Path::new(&self.path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(toml::to_string(&self).unwrap().as_ref())
            .unwrap();
    }

    pub fn read(config_path: &str) -> Self {
        let mut file = File::open(Path::new(config_path)).unwrap();
        let mut data = String::new();

        unsafe {
            file.read_to_end(data.as_mut_vec()).unwrap();
        }

        toml::from_str(&data).unwrap()
    }
}
