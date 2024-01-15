use std::path::Path;
use super::resource;

pub struct Course {
    name: String,
    projects: Vec<resource::ResourceType>,
    labs: Vec<resource::ResourceType>,
    notes: Vec<resource::ResourceType>,
    references: Vec<resource::ResourceType>
}

impl Course {
    /// Creates a folder for the course. This includes subfolders for projects, labs, notes and references
    ///
    /// # Arguments
    ///
    /// * `p0` - The path to the cycle folder
    pub(crate) fn create_folder(&self, p0: &str) {
        let course_path = Path::new(p0).join(self.get_name());
        let course_dir_result = std::fs::create_dir(&course_path);
        if let Err(e) = course_dir_result {
            println!("Failed to create folder: {}", e)
        }

        if let Err(e) = std::fs::create_dir(course_path.join("Projects")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = std::fs::create_dir(course_path.join("Notes")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = std::fs::create_dir(course_path.join("Labs")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = std::fs::create_dir(course_path.join("References")) {
            println!("Failed to create folder: {}", e)
        }
    }
}

impl From<&str> for Course {
    fn from(path: &str) -> Course {
        Course::new(Path::new(path).file_name().unwrap().to_str().unwrap())
    }
}

impl Course {
    pub fn new(name: &str) -> Course {
        Course {
            name: String::from(name),
            projects: Vec::new(),
            labs: Vec::new(),
            notes: Vec::new(),
            references: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Removes the course folder
    ///
    /// # Arguments
    ///
    /// * `p0` - The path to the cycle folder
    pub fn remove_folder(&self, p0: &str) {
        let course_dir_result = std::fs::remove_dir_all(Path::new(p0).join(self.get_name()));
        if let Err(e) = course_dir_result {
            println!("Failed to remove folder: {}", e)
        }
    }

    pub(crate) fn get_projects(&self) -> &Vec<resource::ResourceType> {
        &self.projects
    }
    pub(crate) fn get_labs(&self) -> &Vec<resource::ResourceType> {
        &self.labs
    }
    pub(crate) fn get_notes(&self) -> &Vec<resource::ResourceType> {
        &self.notes
    }
    pub(crate) fn get_references(&self) -> &Vec<resource::ResourceType> {
        &self.references
    }

    /// Loads the resources of the course. This includes projects, labs, notes and references
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the course folder
    pub(crate) fn load_resources(&mut self, path: &str) {
        let paths = std::fs::read_dir(path).expect("Failed to read directory");
        paths.for_each(|path| {
            let path = path.unwrap().path();
            if !path.is_dir() { return; }

            let folder_name = path.file_name().unwrap().to_str().unwrap();
            match folder_name {
                "Projects" => {
                    let projects = std::fs::read_dir(path).unwrap();
                    for project in projects {
                        let project = project.unwrap().path();
                        if project.is_dir() {
                            let project_name = project.file_name().unwrap().to_str().unwrap();
                            self.projects.push(resource::ResourceType::Project {
                                name: String::from(project_name),
                            });
                        }
                    }
                }
                "Labs" => {
                    let labs = std::fs::read_dir(path).unwrap();
                    for lab in labs {
                        let lab = lab.unwrap().path();
                        if lab.is_dir() {
                            let lab_name = lab.file_name().unwrap().to_str().unwrap();
                            self.labs.push(resource::ResourceType::Lab {
                                name: String::from(lab_name),
                            });
                        }
                    }
                }
                "Notes" => {
                    let notes = std::fs::read_dir(path).unwrap();
                    for note in notes {
                        let note = note.unwrap().path();
                        if note.is_dir() {
                            let note_name = note.file_name().unwrap().to_str().unwrap();
                            self.notes.push(resource::ResourceType::Note {
                                name: String::from(note_name),
                            });
                        }
                    }
                }
                "References" => {
                    let references = std::fs::read_dir(path).unwrap();
                    for reference in references {
                        let reference = reference.unwrap().path();
                        if reference.is_dir() {
                            let reference_name = reference.file_name().unwrap().to_str().unwrap();
                            self.references.push(resource::ResourceType::Reference {
                                name: String::from(reference_name),
                            });
                        }
                    }
                }
                _ => {}
            }

        });
    }
}
