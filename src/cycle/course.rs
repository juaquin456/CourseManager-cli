use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Course {
    name: String,
    credits: u8,
}