use std::path::Path;

pub struct Course {
    name: String,
}

impl Course {
    pub(crate) fn create_folder(&self, p0: &str) {
        let create_dir_result = std::fs::create_dir(Path::new(p0).join(self.get_name()));
        if let Err(e) = create_dir_result { println!("Failed to create folder: {}", e) }

        todo!("Create the structure of the course folder");
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
}