use std::{env, fs};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use log::{info, error};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub path: PathBuf,
    pub cycle_prefix: String,
}

pub fn read_config(reconfig: bool) -> Config {
    match home::home_dir() {
        Some(path) => {
            let config_file = path.join(".config").join("CM").join("config.json");
            if config_file.is_file() & !reconfig {
                // Read the file
                info!("Found config file at {}", config_file.display());
                let file = std::fs::File::open(config_file).unwrap();
                let config: Config = serde_json::from_reader(file).unwrap();
                return config;
            } else {
                // Constructing the object
                info!("Creating config file");
                let mut name = String::new();
                println!("What is your name?");
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                let mut pathname = String::new();
                let mut path_docs;
                loop {
                    pathname.clear();
                    println!("Path to manage");
                    std::io::stdin().read_line(&mut pathname).expect("Failed to read line");
                    path_docs = {
                        let tmp_path = pathname.trim();
                        if tmp_path == "." {
                            env::current_dir().unwrap()
                        } else {
                            PathBuf::from(tmp_path)
                        }
                    };
                    if !path_docs.is_dir() {
                        error!("{} is not valid directory", path.display());
                    } else {
                        info!("valid path");
                        break;
                    }
                }
                Config {
                    username: name,
                    path: path_docs,
                    cycle_prefix: String::from("Cycle"),
                }
            }
        }
        None => {
            panic!("Could not find home directory")
        }
    }
}

pub fn write_config(config: &Config) {
    match home::home_dir() {
        Some(path) => {
            info!("Writing config file");
            let mut config_file = path.join(".config").join("CM");
            fs::create_dir_all(config_file.to_str().unwrap()).expect("Failed to create config path");
            config_file.push("config.json");
            let serialized = serde_json::to_string_pretty(config).unwrap();
            std::fs::write(config_file, serialized).unwrap();
        }
        None => {
            panic!("Could not find home directory")
        }
    }
}