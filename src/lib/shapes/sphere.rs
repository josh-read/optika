use super::Shape;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Sphere {
    centre: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Vector3<f64>, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<f64> {
        let oc: Vector3<f64> = ray.origin - self.centre;
        let a: f64 = ray.direction.dot(&ray.direction);
        let b: f64 = 2.0 * oc.dot(&ray.direction);
        let c: f64 = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return None;
        }
        let t1: f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2: f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t = if t1 < f64::EPSILON && t2 < f64::EPSILON {
            return None;
        } else if t1 < f64::EPSILON && t2 >= f64::EPSILON {
            t2
        } else if t1 >= f64::EPSILON && t2 < f64::EPSILON {
            t1
        } else {
            if t1 < t2 {
                t1
            } else {
                t2
            }
        };
        Some(t)
    }

    fn normal(&self, position: Vector3<f64>) -> Vector3<f64> {
        (position - self.centre).normalize()
    }
}