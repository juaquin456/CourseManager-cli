use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Course {
    name: String,
    credits: u8,
}

impl Course {
    pub fn new(name: String, credits: u8) -> Course {
        Course {
            name,
            credits,
        }
    }

    pub fn println(&self) {
        println!("{}: {} credits", self.name, self.credits);
    }
}