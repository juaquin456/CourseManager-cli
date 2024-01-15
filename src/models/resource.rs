#[derive(Debug)]
pub enum ResourceType {
    Project{ name: String },
    Note { name: String },
    Lab { name: String },
    Reference { name: String },
}
