use crate::config::Config;
use crate::{models, parser};

pub fn list(entity: parser::Entities, config: &Config) {
    match entity {
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
    }
}