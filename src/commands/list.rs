use crate::config::Config;
use crate::{models, parser};

pub fn list(entity: parser::Entities, config: &Config) {
    match entity {
        parser::Entities::Cycles => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            cycles
                .iter_mut()
                .for_each(|cycle| {
                    cycle.load_courses(config.get_working_dir());
                    cycle.print(config.get_working_dir());
                });
        }
        parser::Entities::Courses => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            cycles.iter_mut().for_each(|cycle| {
                cycle.load_courses(config.get_working_dir());
                cycle
                    .get_courses()
                    .iter()
                    .for_each(|course| course.print(cycle.get_path(config.get_working_dir()).as_str()));
            });
        }
    }
}