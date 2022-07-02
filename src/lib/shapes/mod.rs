use crate::Ray;
use nalgebra::Vector3;

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<f64> {
        unimplemented!()
    }

    /// As a convention normal should point outwards
    fn normal(&self, position: Vector3<f64>) -> Vector3<f64> {
        unimplemented!()
    }

    fn centre(&self) -> Vector3<f64> {
        unimplemented!()
    }
}

pub mod sphere;

pub use sphere::Sphere;