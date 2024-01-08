use std::fs;
use std::path::Path;
use super::course::Course;

pub struct Cycle {
    age: u16,
    semester: u8,
    courses: Vec<Course>,
}


impl From<&str> for Cycle {
    fn from(folder_name: &str) -> Cycle {
        let (age, semester) = Cycle::get_ids(folder_name);
        Cycle::new(age, semester)
    }
}

impl Cycle {
    pub fn new(age: u16, semester: u8) -> Cycle {
        Cycle {
            age,
            semester,
            courses: Vec::new(),
        }
    }

    pub fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    pub fn get_folder_name(&self) -> String {
        format!("{}-{}", self.age, self.semester)
    }
    pub fn get_courses(&self) -> &Vec<Course> {
        &self.courses
    }

    pub fn get_ids(folder_name: &str) -> (u16, u8) {
        let ids: Vec<&str> = folder_name.split('-').collect();
        let age = ids[0].parse::<u16>().unwrap();
        let semester = ids[1].parse::<u8>().unwrap();
        (age, semester)
    }

    pub(crate) fn create_folder(&self, parent_path: &str) {
        let create_dir_result = fs::create_dir(Path::new(parent_path).join(self.get_folder_name()));
        if let Err(e) = create_dir_result { println!("Failed to create folder: {}", e) }
    }

    pub fn load_cycles(path: &str) -> Vec<Cycle> {
        let mut cycles = Vec::new();
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_str().unwrap();
                cycles.push(Cycle::from(folder_name));
            }
        }
        cycles
    }

    pub(crate) fn load_courses(&mut self, path: &str) {
        let courses_paths = fs::read_dir(Path::new(path).join(self.get_folder_name())).unwrap();
        for course_path in courses_paths {
            let course_path = course_path.unwrap().path();
            if course_path.is_dir() {
                let course = Course::from(course_path.to_str().unwrap());
                self.add_course(course);
            }
        }
    }

    pub fn remove_folder(&self, parent_path: &str) {
        let remove_dir_result = fs::remove_dir_all(Path::new(parent_path).join(self.get_folder_name()));
        if let Err(e) = remove_dir_result { println!("Failed to remove folder: {}", e) }
    }
}