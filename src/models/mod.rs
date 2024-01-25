pub mod course;
pub mod cycle;
pub mod resource;

pub trait Crud
where
    Self: Sized,
{
    fn list(path: &str) -> Vec<Self>;
    fn create(&self);
    fn remove(&self);
}
