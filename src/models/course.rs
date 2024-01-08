use std::path::Path;

pub struct Course {
    name: String,
}

impl Course {
    pub(crate) fn create_folder(&self, p0: &str) {
        let course_path = Path::new(p0).join(self.get_name());
        let course_dir_result = std::fs::create_dir(&course_path);
        if let Err(e) = course_dir_result { println!("Failed to create folder: {}", e) }

        if let Err(e) = std::fs::create_dir(course_path.join("Projects")) { println!("Failed to create folder: {}", e) }
        if let Err(e) = std::fs::create_dir(course_path.join("Notes")) { println!("Failed to create folder: {}", e) }
        if let Err(e) = std::fs::create_dir(course_path.join("Labs")) { println!("Failed to create folder: {}", e) }
        if let Err(e) = std::fs::create_dir(course_path.join("References")) { println!("Failed to create folder: {}", e) }
    }
}

impl From<&str> for Course {
    fn from(path: &str) -> Course {
        Course::new(
            Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        )
    }
}

impl Course {
    pub fn new(name: &str) -> Course {
        Course {
            name: String::from(name),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn remove_folder(&self, p0: &str) {
        let course_dir_result = std::fs::remove_dir_all(Path::new(p0).join(self.get_name()));
        if let Err(e) = course_dir_result { println!("Failed to remove folder: {}", e) }
    }
}