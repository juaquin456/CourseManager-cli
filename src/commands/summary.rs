use crate::config::Config;
use crate::models::Crud;
use crate::{models, parser};

pub fn summary(entity: parser::Entity, config: &Config) {
    match entity {
        parser::Entity::Cycle(cycle) => {
            let cycle_target =
                models::cycle::Cycle::new(cycle.age, cycle.semester, config.get_working_dir());
            let mut cycles = models::cycle::Cycle::list(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|c| c.get_folder_name() == cycle_target.get_folder_name())
                .expect("Cycle not found");

            cycle.load_courses();
            cycle.print_summary();
        }
        parser::Entity::Course(course) => {
            let mut cycles = models::cycle::Cycle::list(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|cycle| cycle.get_folder_name() == course.cycle_id)
                .expect("Cycle not found");

            cycle.load_courses();
            let course_target = models::course::Course::new(&course.name, &cycle.get_path());

            let course = cycle
                .get_courses_mut()
                .iter_mut()
                .find(|c| c.get_name() == course_target.get_name())
                .expect("Course not found");

            course.load_resources();

            course.print_summary();
        }
    }
}
