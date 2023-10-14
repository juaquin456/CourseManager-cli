use clap::Parser;

mod config;
mod cycle;
mod parser;
fn main() {
    env_logger::init();
    let args = parser::Cli::parse();
    let config = config::read_config(args.reconfig);

    match args.command {
        Some(parser::SubCommand::Get{obj}) => {
            match obj {
                parser::Object::Cycle{id} => {
                    cycle::get_cycle(&config, id.expect("Id dont provided"));
                }
                parser::Object::Course{id, name} => {
                    // get_course(&config, id, name);
                }
            }
        }
        Some(parser::SubCommand::Add{obj}) => {
            //add(&config, obj);
        }
        Some(parser::SubCommand::Remove{obj}) => {
            //remove(&config, obj);
        }
        Some(parser::SubCommand::List{obj}) => {
            match obj {
                parser::CollObject::Cycles { } => {
                    cycle::print_cycles(cycle::list_cycles(&config));
                }
                parser::CollObject::Courses { id } => {

                }
            }
        }
        None => {
            println!("No command given");
        }
    }

    config::write_config(&config);
}
