use std::panic;

use clap::Parser;

use config::Config;
use crate::utils::open_terminal;

mod config;
mod models;
mod parser;
mod utils;

fn main() {
    if cfg!(not(debug_assertions)) {
        panic::set_hook(Box::new(|info| {
            let message = match info.payload().downcast_ref::<&str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "No message"
                },
            };

            eprintln!("{}", message);
        }));
    }

    let config = {
        if Config::exists() {
            Config::read(&Config::get_path())
        } else {
            Config::init()
        }
    };

    let cli = parser::Cli::parse();

    match cli.command {
        parser::Commands::List { entity } => match entity {
            parser::Entities::Cycles => {
                let cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                cycles
                    .iter()
                    .for_each(|cycle| println!("{}", cycle.get_folder_name()));
            }
            parser::Entities::Courses => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                cycles.iter_mut().for_each(|cycle| {
                    println!("{}", cycle.get_folder_name());
                    cycle.load_courses(config.get_working_dir());
                    cycle
                        .get_courses()
                        .iter()
                        .for_each(|course| println!("  {}", course.get_name()));
                });
            }
        },
        parser::Commands::Create { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let new_cycle = models::cycle::Cycle::new(cycle.age, cycle.semester);
                new_cycle.create_folder(config.get_working_dir());
            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id)
                    .expect("Cycle not found");

                let new_course = models::course::Course::new(&course.name);
                new_course.create_folder(&format!(
                    "{}/{}",
                    config.get_working_dir(),
                    cycle.get_folder_name()
                ));
                cycle.add_course(new_course);
            }
        },
        parser::Commands::Remove { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let cycle_to_remove = models::cycle::Cycle::new(cycle.age, cycle.semester);
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                cycles.iter_mut()
                    .find(|c| c.get_folder_name() == cycle_to_remove.get_folder_name())
                    .expect("Cycle not found")
                    .remove_folder(config.get_working_dir());

            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id).expect("Cycle not found");

                cycle.load_courses(config.get_working_dir());
                let course_to_remove = models::course::Course::new(&course.name);
                let course = cycle
                    .get_courses()
                    .iter()
                    .find(|c| c.get_name() == course_to_remove.get_name())
                    .expect("Course not found");

                course.remove_folder(&format!(
                    "{}/{}/",
                    config.get_working_dir(),
                    cycle.get_folder_name()
                ));
            }
        },
        parser::Commands::Go { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let cycle_target = models::cycle::Cycle::new(cycle.age, cycle.semester);
                let cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter()
                    .find(|c| c.get_folder_name() == cycle_target.get_folder_name())
                    .expect("Cycle not found");

                match open_terminal(&format!(
                    "{}/{}",
                    config.get_working_dir(),
                    cycle.get_folder_name()
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Failed to open terminal: {}", e);
                    }
                }
            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id).expect("Cycle not found");

                cycle.load_courses(config.get_working_dir());
                let course_target = models::course::Course::new(&course.name);
                let course = cycle
                    .get_courses()
                    .iter()
                    .find(|c| c.get_name() == course_target.get_name()).expect("Course not found");

                match open_terminal(&format!(
                    "{}/{}/{}",
                    config.get_working_dir(),
                    cycle.get_folder_name(),
                    course.get_name()
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Failed to open terminal: {}", e);
                    }
                }
            }
        },
        parser::Commands::Summary { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let cycle_target = models::cycle::Cycle::new(cycle.age, cycle.semester);
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter_mut()
                    .find(|c| c.get_folder_name() == cycle_target.get_folder_name())
                    .expect("Cycle not found");

                cycle.load_courses(config.get_working_dir());
                cycle.print_summary();
            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let cycle = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id).expect("Cycle not found");

                cycle.load_courses(config.get_working_dir());
                let course_target = models::course::Course::new(&course.name);
                let course_folder_name = cycle.get_folder_name();

                let course = cycle
                    .get_courses_mut()
                    .iter_mut()
                    .find(|c| c.get_name() == course_target.get_name()).expect("Course not found");

                course.load_resources(&format!(
                    "{}/{}/{}",
                    config.get_working_dir(),
                    course_folder_name,
                    course.get_name()
                ));

                course.print_summary();
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use crate::parser::{Cli, Commands, Entity, Entities};

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
            _ => panic!("Wrong command"),
        }
    }
}