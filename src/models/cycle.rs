use std::cmp::Ordering;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

use chrono::{DateTime, Utc};

use crate::models::Crud;

use super::course::Course;

pub struct Cycle {
    age: u16,
    semester: u8,
    courses: Vec<Course>,
    parent_path: String,
}

impl Eq for Cycle {}

impl PartialEq<Self> for Cycle {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age && self.semester == other.semester
    }
}

impl PartialOrd<Self> for Cycle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cycle {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.age.cmp(&other.age) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.semester.cmp(&other.semester),
        }
    }
}

impl From<&str> for Cycle {
    fn from(folder_name: &str) -> Self {
        let path = Path::new(folder_name);
        let (age, semester) = Cycle::get_ids(path.file_name().unwrap().to_str().unwrap()).unwrap();
        Cycle::new(age, semester, path.parent().unwrap().to_str().unwrap())
    }
}


impl Cycle {
    pub fn new(age: u16, semester: u8, parent_path: &str) -> Cycle {
        Cycle {
            age,
            semester,
            courses: Vec::new(),
            parent_path: String::from(parent_path),
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

    pub fn get_courses_mut(&mut self) -> &mut Vec<Course> {
        &mut self.courses
    }

    /// Returns a tuple containing the age and the semester of the cycle
    ///
    /// # Arguments
    ///
    /// * `folder_name` - The name of the folder containing the cycle
    pub fn get_ids(folder_name: &str) -> Result<(u16, u8), ParseIntError> {
        let ids: Vec<&str> = folder_name.split('-').collect();
        let age = ids[0].parse::<u16>()?;
        let semester = ids[1].parse::<u8>()?;
        Ok((age, semester))
    }

    pub fn get_path(&self) -> String {
        Path::new(&self.parent_path)
            .join(self.get_folder_name())
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Loads all the courses from the folder path given
    pub(crate) fn load_courses(&mut self) {
        let courses_paths =
            fs::read_dir(Path::new(&self.parent_path).join(self.get_folder_name())).unwrap();
        for course_path in courses_paths {
            let course_path = course_path.unwrap().path();
            if course_path.is_dir() {
                let course = Course::from(course_path.to_str().unwrap());
                self.add_course(course);
            }
        }
    }

    pub(crate) fn print_summary(&self) {
        println!("Cycle {}:", self.get_folder_name());
        self.get_courses().iter().for_each(|course| {
            println!("  {}", course.get_name());
        });
    }
}

impl Display for Cycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wd = Path::new(&self.parent_path);
        let metadata = fs::metadata(wd.join(self.get_folder_name())).unwrap();
        let created_at: DateTime<Utc> = DateTime::from(metadata.created().unwrap());
        write!(
            f,
            "{} {:>2} courses {:<12}",
            self.get_folder_name(),
            self.get_courses().len(),
            created_at.format("%d/%m/%Y")
        )
    }
}

impl Crud for Cycle {
    /// Loads all the cycles from the folder path given
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the folder containing the cycles
    ///
    /// # Returns
    ///
    /// A vector containing all the cycles
    fn list(path: &str) -> Vec<Self> {
        let mut cycles = Vec::new();
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_str().unwrap();
                match Cycle::get_ids(folder_name) {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                cycles.push(Cycle::from(path.to_str().unwrap()));
            }
        }
        cycles.sort();
        cycles
    }

    /// Creates a folder for the cycle. The folder name is the concatenation of the age and the semester of the cycle.
    /// Check the `get_folder_name` method for more details.
    fn create(&self) {
        let create_dir_result =
            fs::create_dir(Path::new(&self.parent_path).join(self.get_folder_name()));
        if let Err(e) = create_dir_result {
            println!("Failed to create folder: {}", e)
        }
    }

    /// Removes the folder of the cycle
    fn remove(&self) {
        let remove_dir_result =
            fs::remove_dir_all(Path::new(&self.parent_path).join(self.get_folder_name()));
        if let Err(e) = remove_dir_result {
            println!("Failed to remove folder: {}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let cycle = Cycle::new(1, 2, "/tmp");
        assert_eq!(cycle.age, 1);
        assert_eq!(cycle.semester, 2);
    }

    #[test]
    fn test_get_folder_name() {
        let cycle = Cycle::new(1, 2, "/tmp");
        assert_eq!(cycle.get_folder_name(), "1-2");
    }

    #[test]
    fn test_get_ids() {
        let (age, semester) = Cycle::get_ids("1-2").unwrap();
        assert_eq!(age, 1);
        assert_eq!(semester, 2);
    }

    #[test]
    fn test_create_folder() {
        let cycle = Cycle::new(1, 2, "/tmp");
        cycle.create();
        assert!(Path::new("/tmp/1-2").exists());
        fs::remove_dir_all("/tmp/1-2").unwrap();
    }

    #[test]
    fn test_load_cycles() {
        fs::create_dir_all("/tmp/t1").unwrap();

        let cycle = Cycle::new(1, 3, "/tmp/t1");
        cycle.create();
        let cycles = Cycle::list("/tmp/t1");

        assert!(Path::new("/tmp/t1/1-3").exists());
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].age, 1);
        assert_eq!(cycles[0].semester, 3);

        fs::remove_dir_all("/tmp/t1/1-3").unwrap();
    }

    #[test]
    fn test_remove_folder() {
        let cycle = Cycle::new(1, 2, "/tmp");
        cycle.create();
        cycle.remove();
        assert!(!Path::new("/tmp/1-2").exists());
    }
}
