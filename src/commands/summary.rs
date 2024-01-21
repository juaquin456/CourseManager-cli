use crate::config::Config;
use crate::{models, parser};

pub fn summary(entity: parser::Entity, config: &Config) {
    match entity {
        parser::Entity::Cycle(cycle) => {
            let cycle_target = models::cycle::Cycle::new(cycle.age, cycle.semester);
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|c| c.get_folder_name() == cycle_target.get_folder_name())
                .expect("Cycle not found");

            cycle.load_courses(config.get_working_dir());
            cycle.print_summary();
        }
        parser::Entity::Course(course) => {
            let mut cycles = models::cycle::Cycle::load_cycles(config.get_working_dir());
            let cycle = cycles
                .iter_mut()
                .find(|cycle| cycle.get_folder_name() == course.cycle_id).expect("Cycle not found");

            cycle.load_courses(config.get_working_dir());
            let course_target = models::course::Course::new(&course.name);
            let course_folder_name = cycle.get_folder_name();

            let course = cycle
                .get_courses_mut()
                .iter_mut()
                .find(|c| c.get_name() == course_target.get_name()).expect("Course not found");

            course.load_resources(&format!(
                "{}/{}/{}",
                config.get_working_dir(),
                course_folder_name,
                course.get_name()
            ));

            course.print_summary();
        }
    }
}