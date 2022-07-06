use approx::ulps_eq;
use crate::{Ray, Shape};
use nalgebra::Vector3;

#[derive(Clone, Copy)]
pub enum PlaneBounds {
    Circular { radius: f64 },
    Rectangular { width: f64, height: f64 },
    None,
}

#[derive(Clone, Copy)]
pub struct Plane {
    pub centre: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub bounds: PlaneBounds,
}

impl Plane {
    pub fn new(centre: Vector3<f64>, normal: Vector3<f64>) -> Plane {
        Plane {
            centre,
            normal,
            bounds: PlaneBounds::None,
        }
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.bounds = PlaneBounds::Circular { radius };
        self
    }
}

impl Shape for Plane {

    fn intersection(&self, ray: &Ray) -> Option<f64> {
        let n = self.normal;
        let d = ray.direction;
        let ray_projection_along_normal = n.dot(&d);
        if ulps_eq!(ray_projection_along_normal, 0.0) {
            return None;
        }
        let numerator = (self.centre - ray.origin).dot(&n);
        let t = numerator / ray_projection_along_normal;
        if t <= 0.0 {
            return None;
        }
        match self.bounds {
            PlaneBounds::None => Some(t),
            PlaneBounds::Circular { radius } => {
                if (ray.position_at(t) - self.centre).norm() < radius {
                    Some(t)
                } else {
                    None
                }
            }
            _ => unimplemented!(),
        }
    }

    fn normal(&self, _position: Vector3<f64>) -> Vector3<f64> {
        self.normal
    }

    fn centre(&self) -> Vector3<f64> {
        self.centre
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use super::*;

    fn setup() -> Plane {
        Plane::new(100.0 * FORWARD, BACKWARD).with_radius(25.0)
    }

    #[test]
    fn test_hit() {
        let p = setup();
        let res = p.intersection(&AXIAL);
        assert_eq!(res, Some(100.0));
    }

    #[test]
    fn test_miss() {
        let p = setup();
        let res = p.intersection(&(-AXIAL));
        assert_eq!(res, None);
    }

}