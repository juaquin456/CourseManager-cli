use std::path::Path;
use super::course::Course;

pub struct Cycle {
    path: Box<Path>,
    age: u16,
    semester: u8,
    courses: Vec<Course>,
}

impl From<&str> for Cycle {
    fn from(folder_name: &str) -> Cycle {
        let (age, semester) = Cycle::get_ids(folder_name);
        Cycle::new(Box::from(Path::new(folder_name)), age, semester)
    }
}

impl Cycle {
    pub fn new(path: Box<Path>, age: u16, semester: u8) -> Cycle {
        Cycle {
            path,
            age,
            semester,
            courses: Vec::new(),
        }
    }

    pub fn add_course(&mut self, course: Box<Course>) {
        self.courses.push(*course);
    }

    pub fn get_folder_name(&self) -> String {
        format!("{}-{}", self.age, self.semester)
    }

    pub fn get_ids(folder_name: &str) -> (u16, u8) {
        let ids: Vec<&str> = folder_name.split("-").collect();
        let age = ids[0].parse::<u16>().unwrap();
        let semester = ids[1].parse::<u8>().unwrap();
        (age, semester)
    }
}