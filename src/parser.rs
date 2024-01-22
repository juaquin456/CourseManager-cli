use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub working_dir: Option<String>,
}

#[derive(Args, Clone)]
pub struct Cycle {
    pub age: u16,
    pub semester: u8,
}

#[derive(Args)]
pub struct Course {
    pub cycle_id: String,
    pub name: String,

    #[arg(short, long)]
    pub resource: Option<CourseResources>,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[command(subcommand)]
        entity: Entity,
    },
    Remove {
        #[command(subcommand)]
        entity: Entity,
    },
    List {
        #[arg(value_enum)]
        entity: Entities,
    },
    #[command(about = "Open the resource folder in a new terminal window")]
    Go {
        #[command(subcommand)]
        entity: Entity,
    },
    #[command(about = "Print a summary of the specified resource")]
    Summary {
        #[command(subcommand)]
        entity: Entity,
    },
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Entities {
    Cycles,
    Courses,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum CourseResources {
    Projects,
    Labs,
    References,
    Notes,
}

#[derive(Subcommand)]
pub enum Entity {
    Cycle(Cycle),
    Course(Course),
}

#[cfg(test)]
mod tests {
    use super::*;
    impl Cycle {
        pub(crate) fn parse_from(p0: &[&str; 3]) -> Self {
            Cycle {
                age: p0[1].parse::<u16>().unwrap(),
                semester: p0[2].parse::<u8>().unwrap(),
            }
        }
    }

    impl Course {
        pub(crate) fn parse_from(p0: &[&str; 3]) -> Self {
            Self {
                cycle_id: p0[1].to_string(),
                name: p0[2].to_string(),
                resource: None,
            }
        }
    }
    #[test]
    fn test_cycle() {
        let cycle = Cycle::parse_from(&["cycle", "1", "2"]);
        assert_eq!(cycle.age, 1);
        assert_eq!(cycle.semester, 2);
    }

    #[test]
    fn test_course() {
        let course = Course::parse_from(&["course", "1-2", "course_name"]);
        assert_eq!(course.cycle_id, "1-2");
        assert_eq!(course.name, "course_name");
    }

    #[test]
    fn test_create_cycle() {
        let cli = Cli::parse_from(["author", "create", "cycle", "1", "2"]);
        match cli.command {
            Commands::Create { entity } => match entity {
                Entity::Cycle(cycle) => {
                    assert_eq!(cycle.age, 1);
                    assert_eq!(cycle.semester, 2);
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_create_course() {
        let cli = Cli::parse_from(["author", "create", "course", "1-2", "course_name"]);
        match cli.command {
            Commands::Create { entity } => match entity {
                Entity::Course(course) => {
                    assert_eq!(course.cycle_id, "1-2");
                    assert_eq!(course.name, "course_name");
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_list_cycles() {
        let cli = Cli::parse_from(["author", "list", "cycles"]);
        match cli.command {
            Commands::List { entity } => match entity {
                Entities::Cycles => {}
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_remove_cycle() {
        let cli = Cli::parse_from(["author", "remove", "cycle", "1", "2"]);
        match cli.command {
            Commands::Remove { entity } => match entity {
                Entity::Cycle(cycle) => {
                    assert_eq!(cycle.age, 1);
                    assert_eq!(cycle.semester, 2);
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong"),
        }
    }

    #[test]
    fn test_list_courses() {
        let cli = Cli::parse_from(["author", "list", "courses"]);
        match cli.command {
            Commands::List { entity } => match entity {
                Entities::Courses => {}
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_go_cycle() {
        let cli = Cli::parse_from(["author", "go", "cycle", "1", "2"]);
        match cli.command {
            Commands::Go { entity } => match entity {
                Entity::Cycle(cycle) => {
                    assert_eq!(cycle.age, 1);
                    assert_eq!(cycle.semester, 2);
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_go_course() {
        let cli = Cli::parse_from(["author", "go", "course", "1-2", "course_name"]);
        match cli.command {
            Commands::Go { entity } => match entity {
                Entity::Course(course) => {
                    assert_eq!(course.cycle_id, "1-2");
                    assert_eq!(course.name, "course_name");
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_summary_cycle() {
        let cli = Cli::parse_from(["author", "summary", "cycle", "1", "2"]);
        match cli.command {
            Commands::Summary { entity } => match entity {
                Entity::Cycle(cycle) => {
                    assert_eq!(cycle.age, 1);
                    assert_eq!(cycle.semester, 2);
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_summary_course() {
        let cli = Cli::parse_from(["author", "summary", "course", "1-2", "course_name"]);
        match cli.command {
            Commands::Summary { entity } => match entity {
                Entity::Course(course) => {
                    assert_eq!(course.cycle_id, "1-2");
                    assert_eq!(course.name, "course_name");
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }

    #[test]
    fn test_remove_course() {
        let cli = Cli::parse_from(["author", "remove", "course", "1-2", "course_name"]);
        match cli.command {
            Commands::Remove { entity } => match entity {
                Entity::Course(course) => {
                    assert_eq!(course.cycle_id, "1-2");
                    assert_eq!(course.name, "course_name");
                }
                _ => panic!("Wrong entity"),
            },
            _ => panic!("Wrong command"),
        }
    }
}