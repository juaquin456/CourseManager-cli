use clap::Parser;
use std::fs;
use std::process::Command;

mod config;
mod models;
mod parser;
use config::Config;

fn main() {
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
                println!("The path you entered does not exist");
            } else {
                config = Config::new(fs::canonicalize(path).unwrap().to_str().unwrap());
                break;
            }
        }

        config.write();
    } else {
        config = Config::read(&Config::get_path());
    }

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
                let res = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id);
                match res {
                    Some(cycle) => {
                        let new_course = models::course::Course::new(&course.name);
                        new_course.create_folder(&format!(
                            "{}/{}",
                            config.get_working_dir(),
                            cycle.get_folder_name()
                        ));
                        cycle.add_course(new_course);
                    }
                    None => {
                        println!("Cycle {} not found", course.cycle_id);
                    }
                }
            }
        },
        parser::Commands::Remove { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let cycle_to_remove = models::cycle::Cycle::new(cycle.age, cycle.semester);
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let res = cycles
                    .iter_mut()
                    .find(|c| c.get_folder_name() == cycle_to_remove.get_folder_name());
                match res {
                    Some(cycle) => {
                        cycle.remove_folder(config.get_working_dir());
                    }
                    None => {
                        println!("Cycle {} not found", cycle_to_remove.get_folder_name());
                    }
                }
            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let res = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id);
                match res {
                    Some(cycle) => {
                        cycle.load_courses(config.get_working_dir());
                        let course_to_remove = models::course::Course::new(&course.name);
                        let res = cycle
                            .get_courses()
                            .iter()
                            .find(|c| c.get_name() == course_to_remove.get_name());
                        match res {
                            Some(course) => {
                                course.remove_folder(&format!(
                                    "{}/{}/",
                                    config.get_working_dir(),
                                    cycle.get_folder_name()
                                ));
                            }
                            None => {
                                println!("Course {} not found", course_to_remove.get_name());
                            }
                        }
                    }
                    None => {
                        println!("Cycle {} not found", course.cycle_id);
                    }
                }
            }
        },
        parser::Commands::Go { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                let cycle_target = models::cycle::Cycle::new(cycle.age, cycle.semester);
                let cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let res = cycles
                    .iter()
                    .find(|c| c.get_folder_name() == cycle_target.get_folder_name());
                match res {
                    Some(cycle) => {
                        if cfg!(target_os = "linux") {
                            Command::new("gnome-terminal")
                                .arg("--working-directory")
                                .arg(format!(
                                    "{}/{}",
                                    config.get_working_dir(),
                                    cycle.get_folder_name()
                                ))
                                .output()
                                .expect("failed to execute process")
                        } else {
                            panic!("Unsupported OS")
                        };
                    }
                    None => {
                        println!("Cycle {} not found", cycle_target.get_folder_name());
                    }
                }
            }
            parser::Entity::Course(course) => {
                let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
                let res = cycles
                    .iter_mut()
                    .find(|cycle| cycle.get_folder_name() == course.cycle_id);
                match res {
                    Some(cycle) => {
                        cycle.load_courses(config.get_working_dir());
                        let course_target = models::course::Course::new(&course.name);
                        let res = cycle
                            .get_courses()
                            .iter()
                            .find(|c| c.get_name() == course_target.get_name());
                        match res {
                            Some(course) => {
                                if cfg!(target_os = "linux") {
                                    Command::new("gnome-terminal")
                                        .arg("--working-directory")
                                        .arg(format!(
                                            "{}/{}/{}",
                                            config.get_working_dir(),
                                            cycle.get_folder_name(),
                                            course.get_name()
                                        ))
                                        .output()
                                        .expect("failed to execute process")
                                } else {
                                    panic!("Unsupported OS")
                                };
                            }
                            None => {
                                println!("Course {} not found", course_target.get_name());
                            }
                        }
                    }
                    None => {
                        println!("Cycle {} not found", course.cycle_id);
                    }
                }
            }
        },
        parser::Commands::Summary { entity } => match entity {
            parser::Entity::Cycle(cycle) => {
                println!("Getting summary of cycle {}-{}", cycle.age, cycle.semester);
            }
            parser::Entity::Course(course) => {
                println!(
                    "Getting summary of course {} {}",
                    course.cycle_id, course.name
                );
            }
        },
    }
}
