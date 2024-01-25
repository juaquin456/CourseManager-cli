use super::{resource, Crud};
use chrono::{DateTime, Utc};
use std::fmt::Display;
use std::fs;
use std::path::Path;

pub struct Course {
    name: String,
    parent_path: String,
    projects: Vec<resource::ResourceType>,
    labs: Vec<resource::ResourceType>,
    notes: Vec<resource::ResourceType>,
    references: Vec<resource::ResourceType>,
}

impl From<&str> for Course {
    fn from(path: &str) -> Course {
        let path = Path::new(path);
        Course::new(
            path.file_name().unwrap().to_str().unwrap(),
            path.parent().unwrap().to_str().unwrap(),
        )
    }
}

impl Course {
    pub fn new(name: &str, parent_path: &str) -> Course {
        Course {
            name: String::from(name),
            projects: Vec::new(),
            labs: Vec::new(),
            notes: Vec::new(),
            references: Vec::new(),
            parent_path: String::from(parent_path),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_path(&self) -> String {
        Path::new(&self.parent_path)
            .join(self.get_name())
            .to_str()
            .unwrap()
            .to_string()
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
    pub(crate) fn load_resources(&mut self) {
        let paths = fs::read_dir(&self.parent_path).expect("Failed to read directory");
        paths.for_each(|path| {
            let path = path.unwrap().path();
            if !path.is_dir() {
                return;
            }

            let folder_name = path.file_name().unwrap().to_str().unwrap();
            match folder_name {
                "Projects" => {
                    let projects = fs::read_dir(path).unwrap();
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
                    let labs = fs::read_dir(path).unwrap();
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
                    let notes = fs::read_dir(path).unwrap();
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
                    let references = fs::read_dir(path).unwrap();
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

    pub(crate) fn print_summary(&self) {
        println!("Summary of course {}:", self.get_name());
        println!("\tProjects\n\t\t{:?}", self.get_projects());
        println!("\tNotes\n\t\t{:?}", self.get_notes());
        println!("\tLabs\n\t\t{:?}", self.get_labs());
        println!("\tReferences\n\t\t{:?}", self.get_references());
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cycle_path = Path::new(&self.parent_path);

        let cycle_id = cycle_path.file_name().unwrap().to_str().unwrap();
        let metadata = fs::metadata(cycle_path.join(self.get_name())).unwrap();
        let created_at: DateTime<Utc> = DateTime::from(metadata.created().unwrap());

        write!(
            f,
            "{:10} {} {:<12}",
            self.get_name(),
            cycle_id,
            created_at.format("%d/%m/%Y")
        )
    }
}

impl Crud for Course {
    /// Loads all the courses from the folder path given
    fn list(path: &str) -> Vec<Self> {
        let courses_paths = fs::read_dir(path).unwrap();
        let mut res = Vec::new();

        for course_path in courses_paths {
            let course_path = course_path.unwrap().path();
            if course_path.is_dir() {
                let course = Course::from(course_path.to_str().unwrap());
                res.push(course);
            }
        }
        res
    }

    /// Creates a folder for the course. This includes sub folders for projects, labs, notes and references
    fn create(&self) {
        let course_path = Path::new(&self.parent_path).join(self.get_name());
        let course_dir_result = fs::create_dir(&course_path);
        if let Err(e) = course_dir_result {
            println!("Failed to create folder: {}", e)
        }

        if let Err(e) = fs::create_dir(course_path.join("Projects")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = fs::create_dir(course_path.join("Notes")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = fs::create_dir(course_path.join("Labs")) {
            println!("Failed to create folder: {}", e)
        }
        if let Err(e) = fs::create_dir(course_path.join("References")) {
            println!("Failed to create folder: {}", e)
        }
    }

    /// Removes the folder of the course
    fn remove(&self) {
        let course_dir_result =
            fs::remove_dir_all(Path::new(&self.parent_path).join(self.get_name()));
        if let Err(e) = course_dir_result {
            println!("Failed to remove folder: {}", e)
        }
    }
}
