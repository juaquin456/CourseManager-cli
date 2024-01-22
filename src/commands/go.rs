use crate::config::Config;
use crate::{models, parser};
use crate::utils::open_terminal;

pub fn go(entity: parser::Entity, config: &Config) {
    match entity {
        parser::Entity::Cycle(cycle) => {
            let cycle_target = models::cycle::Cycle::new(cycle.age, cycle.semester, config.get_working_dir());
            let cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter()
                .find(|c| c.get_folder_name() == cycle_target.get_folder_name())
                .expect("Cycle not found");

            match open_terminal(&format!(
                "{}/{}",
                config.get_working_dir(),
                cycle.get_folder_name()
            )) {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to open terminal: {}", e);
                }
            }
        }
        parser::Entity::Course(course) => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|cycle| cycle.get_folder_name() == course.cycle_id).expect("Cycle not found");

            cycle.load_courses();
            let course_target = models::course::Course::new(&course.name, &cycle.get_path());

            let course_t = cycle
                .get_courses()
                .iter()
                .find(|c| c.get_name() == course_target.get_name()).expect("Course not found");

            let path = {
                if let Some(t) = course.resource {
                    format!(
                        "{}/{:?}",
                        course_t.get_path(),
                        t
                    )
                } else {
                    course_t.get_path()
                }
            };

            match open_terminal(&path) {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to open terminal: {}", e);
                }
            }
        }
    }
}