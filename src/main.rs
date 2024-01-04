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
                    println!("Creating course {}-{} {}", course.age, course.semester, course.name);
                }
            }
        },
        parser::Commands::Remove {entity} => {
            match entity {
                parser::Entity::Cycle(cycle) => {
                    println!("Removing cycle {}-{}", cycle.age, cycle.semester);
                },
                parser::Entity::Course(course) => {
                    println!("Removing course {}-{} {}", course.age, course.semester, course.name);
                }
            }
        },
    }
}