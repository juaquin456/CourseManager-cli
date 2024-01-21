use crate::config::Config;
use crate::{models, parser};

pub fn create(entity: parser::Entity, config: &Config) {
    match entity {
        parser::Entity::Cycle(cycle) => {
            let new_cycle = models::cycle::Cycle::new(cycle.age, cycle.semester);
            new_cycle.create_folder(config.get_working_dir());
        }
        parser::Entity::Course(course) => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|cycle| cycle.get_folder_name() == course.cycle_id)
                .expect("Cycle not found");

            let new_course = models::course::Course::new(&course.name);
            new_course.create_folder(&format!(
                "{}/{}",
                config.get_working_dir(),
                cycle.get_folder_name()
            ));
            cycle.add_course(new_course);
        }
    }
}