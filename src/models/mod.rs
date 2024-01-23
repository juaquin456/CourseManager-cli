pub mod course;
pub mod cycle;
pub mod resource;

pub trait Crud {
    fn create(&self);
    fn remove(&self);
}
