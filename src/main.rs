use std::{fs, panic};

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

    let config;
    if !Config::exists() {
        println!("Config file not found, creating one...");
        loop {
            let mut input = String::new();
            println!("Enter the path to the working directory:");
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let path = std::path::Path::new(input);
            if !path.is_dir() & !path.is_file() {
                eprintln!("The path you entered does not exist");
            } else {
                config = Config::new(fs::canonicalize(path).unwrap().to_str().unwrap());
                break;
            }
        }

        config.write();
    } else { config = Config::read(&Config::get_path()); }

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
                println!("Summary of cycle {}:", cycle.get_folder_name());
                cycle.get_courses().iter().for_each(|course| {
                    println!("  {}", course.get_name());
                });
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

                println!("Summary of course {}:", course.get_name());
                println!("\tProjects\n\t\t{:?}", course.get_projects());
                println!("\tNotes\n\t\t{:?}", course.get_notes());
                println!("\tLabs\n\t\t{:?}", course.get_labs());
                println!("\tReferences\n\t\t{:?}", course.get_references());
            }
        },
    }
}
