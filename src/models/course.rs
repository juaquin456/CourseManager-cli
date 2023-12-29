use std::path::Path;

pub struct Course {
    name: String,
    path: Box<Path>,
}

impl From<&str> for Course {
    fn from(path: &str) -> Course {
        Course::new(
            Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            Box::from(Path::new(path))
        )
    }
}

impl Course {
    pub fn new(name: &str, path: Box<Path>) -> Course {
        Course {
            name: String::from(name),
            path,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}