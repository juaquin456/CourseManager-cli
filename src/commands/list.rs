use crate::config::Config;
use crate::{models, parser};

pub fn list(entity: parser::Entities, config: &Config) {
    match entity {
        parser::Entities::Cycles => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            cycles.iter_mut().for_each(|cycle| {
                cycle.load_courses();
                println!("{}", cycle);
            });
        }
        parser::Entities::Courses => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            cycles.iter_mut().for_each(|cycle| {
                cycle.load_courses();
                cycle
                    .get_courses()
                    .iter()
                    .for_each(|course| println!("{}", course));
            });
        }
    }
}
