use crate::config::Config;
use crate::models::Crud;
use crate::{models, parser};

pub fn create(entity: parser::Entity, config: &Config) {
    match entity {
        parser::Entity::Cycle(cycle) => {
            let new_cycle =
                models::cycle::Cycle::new(cycle.age, cycle.semester, config.get_working_dir());
            new_cycle.create();
        }
        parser::Entity::Course(course) => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|cycle| cycle.get_folder_name() == course.cycle_id)
                .expect("Cycle not found");

            let new_course = models::course::Course::new(&course.name, &cycle.get_path());
            new_course.create();
            cycle.add_course(new_course);
        }
    }
}
