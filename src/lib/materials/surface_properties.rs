use super::*;
use crate::*;
use nalgebra::Vector3;
use approx::ulps_eq;

#[derive(Debug, Clone, Copy)]
pub enum RayBehaviour {
    Reflect,
    Absorb,
    Transmit,
}

#[derive(Debug, Clone, Copy)]
pub enum DielectricProperties {
    Constant(f64),
    ThinLens(f64),
}

pub struct SurfaceProperties {
    pub primary_behaviour: RayBehaviour,
    pub reflectance: f64,
    pub absorption: f64,
    pub transmittance: f64,
    pub dielectric_properties: DielectricProperties,
}

impl SurfaceProperties {

    pub fn new() -> SurfaceBuilder {
        SurfaceBuilder::default()
    }

    // Could potentially reduce run time by removing duplicate ray_intersection_position calculation
    // (here and in parent function (in impl OpticalElement)) - maybe with closure argument.
    pub fn refract(&self, input_ray: Ray, t: f64, shape: &Box<dyn Shape>) -> Ray {
        use DielectricProperties::*;
        let ray_intersection_position: Vector3<f64> = input_ray.position_at(t);
        let normal = shape.normal(ray_intersection_position);
        match self.dielectric_properties {
            Constant(n) => {
                let etai_over_etat = input_ray.n / n;
                let ray_refracted_direction: Vector3<f64> = refract(input_ray.direction, normal, etai_over_etat);
                Ray::new(ray_intersection_position, ray_refracted_direction, n)
            },
            ThinLens(f) => {
                let centre = shape.centre();
                let intersection_distance_from_centre: f64 =
                    (ray_intersection_position - centre).norm();
                if ulps_eq!(intersection_distance_from_centre, 0.0) {
                    return Ray::new(ray_intersection_position, input_ray.direction, input_ray.n);
                }
                let intersection_angle: f64 =
                    (input_ray.direction.dot(&normal)).acos();
                let new_angle = intersection_angle / f;
                let new_direction_perpendicular =
                    new_angle.sin() * (ray_intersection_position - centre).normalize();
                let new_direction_parallel = new_angle.cos() * centre;
                let new_direction = new_direction_perpendicular + new_direction_parallel;
                Ray::new(ray_intersection_position, new_direction.normalize(), input_ray.n)
            }
        }
        
    }
}