use clap::Parser;

mod models;
mod parser;

fn main() {
    let cli = parser::Cli::parse();

    match cli.command {
        parser::Commands::List { entity } => {
            match entity {
                parser::Entities::Cycles => {
                    println!("Listing cycles");
                },
                parser::Entities::Courses => {
                    println!("Listing courses");
                }
            }
        },
        parser::Commands::Create {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    println!("Creating cycle {}-{}", cycle.age, cycle.semester);
                },
                parser::Entity::Course(course) => {
                    println!("Creating course {} {}", course.cycle_id, course.name);
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