use std::fs;

use serde::{Deserialize, Serialize};

use crate::config::Config;

mod course;

#[derive(Serialize, Deserialize)]
pub struct Cycle {
    id: u16,
    #[serde(skip)]
    courses: Vec<course::Course>,
}


impl Cycle {
    pub fn new(id: u16) -> Cycle {
        Cycle {
            id,
            courses: Vec::new(),
        }
    }

    pub fn println(&self) {
        print!("Cycle {}\n", self.id);
        for course in &self.courses {
            course.println();
        }
    }
}

pub fn get_cycle(_config: &Config, _id: u16) -> Cycle {
    Cycle {id:1, courses: vec![]}
}

pub fn list_cycles(config: &Config) -> Vec<Cycle> {
    let mut res = Vec::new();
    for path in fs::read_dir(config.path.as_path()).unwrap() {
        let name = path.unwrap().file_name();
        if name.to_str().unwrap()[..config.cycle_prefix.len()] == config.cycle_prefix {
            log::info!("Readed dir {}", name.to_str().unwrap());
            let last_digits = name.len() - config.cycle_prefix.len();
            let c =  &name.to_str().unwrap()[(name.len() - last_digits)..];
            res.push(Cycle::new(c.parse().expect(format!("cannot parse the {c}").as_str())));
        }
    }
    
    res
}

pub fn print_cycles(cycles: Vec<Cycle>) {
    for cycle in cycles {
        cycle.println();
    }
}