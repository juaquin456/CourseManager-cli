use std::path::Path;

pub struct Course {
    name: String,
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
}