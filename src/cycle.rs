use serde::{Deserialize, Serialize};

mod course;

#[derive(Serialize, Deserialize)]
struct Cycle {
    id: u16,
    #[serde(skip)]
    courses: Vec<course::Course>,
}

fn get_cycle(Config: String, id: u16) -> Cycle {
}