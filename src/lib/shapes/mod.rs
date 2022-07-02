use crate::Ray;
use nalgebra::Vector3;

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<f64>;

    /// As a convention normal should point outwards
    fn normal(&self, position: Vector3<f64>) -> Vector3<f64>;

    fn centre(&self) -> Vector3<f64> {
        unimplemented!()
    }
}

pub mod sphere;
pub mod plane;

pub use sphere::Sphere;
pub use plane::Plane;