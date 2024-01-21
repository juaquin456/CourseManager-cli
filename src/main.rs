use std::panic;

use clap::Parser;

use config::Config;

mod config;
mod models;
mod parser;
mod utils;

mod commands;

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
        parser::Commands::List { entity } => {
            commands::list(entity, &config);
        },
        parser::Commands::Create { entity } => {
            commands::create(entity, &config);
        },
        parser::Commands::Remove { entity } => {
            commands::remove(entity, &config);
        }
        parser::Commands::Go { entity } => {
            commands::go(entity, &config);
        },
        parser::Commands::Summary { entity } => {
            commands::summary(entity, &config);
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