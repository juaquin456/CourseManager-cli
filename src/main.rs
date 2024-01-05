use std::fs;
use clap::Parser;

mod models;
mod parser;
mod config;
use config::Config;

fn main() {
    let config ;
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
            }
            else {
                config = Config::new(fs::canonicalize(path).unwrap().to_str().unwrap());
                break;
            }
        }

        config.write();
    }
    else {
        config = Config::read(&Config::get_path());
    }

    let cli = parser::Cli::parse();


    match cli.command {
        parser::Commands::List { entity } => {
            match entity {
                parser::Entities::Cycles => {
                    let cycles = models::cycle::Cycle::read_cycles_folder(config.get_working_dir());
                    cycles.iter().for_each(|cycle| println!("{}", cycle.get_folder_name()));
                },
                parser::Entities::Courses => {
                    println!("Listing courses");
                }
            }
        },
        parser::Commands::Create {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    let new_cycle = models::cycle::Cycle::new(cycle.age, cycle.semester);
                    new_cycle.create_folder(config.get_working_dir());
                },
                parser::Entity::Course(course) => {
                    let mut cycles = models::cycle::Cycle::read_cycles_folder(config.get_working_dir());
                    let res = cycles.iter_mut().find(|cycle| cycle.get_folder_name() == course.cycle_id);
                    match res {
                        Some(cycle) => {
                            let new_course = models::course::Course::new(&course.name);
                            new_course.create_folder(&format!("{}/{}", config.get_working_dir(), cycle.get_folder_name()));
                            cycle.add_course(new_course);
                        },
                        None => {
                            println!("Cycle {} not found", course.cycle_id);
                        }
                    }
                }
            }
        },
        parser::Commands::Remove {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    println!("Removing cycle {}-{}", cycle.age, cycle.semester);
                },
                parser::Entity::Course(course) => {
                    println!("Removing course {} {}", course.cycle_id, course.name);
                }
            }
        },
        parser::Commands::Go {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    println!("Going to cycle {}-{}", cycle.age, cycle.semester);
                },
                parser::Entity::Course(course) => {
                    println!("Going to course {} {}", course.cycle_id, course.name);
                }
            }
        },
        parser::Commands::Summary {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    println!("Getting summary of cycle {}-{}", cycle.age, cycle.semester);
                },
                parser::Entity::Course(course) => {
                    println!("Getting summary of course {} {}", course.cycle_id, course.name);
                }
            }
        }
    }
}