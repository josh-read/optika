use crate::{Shape, Plane, SurfaceProperties, Ray, materials::{RayBehaviour, reflect}};
use nalgebra::Vector3;

pub struct OpticalElement {
    pub shape: Box<dyn Shape>,
    pub surface_properties: SurfaceProperties,
}

impl OpticalElement {
    
    /// Produce a new OpticalElement
    pub fn new(shape: Box<dyn Shape>, surface_properties: SurfaceProperties) -> OpticalElement {
        OpticalElement {shape, surface_properties}
    }

    /// Produce a new OpticalElement with properties of a thin lens
    pub fn new_thin_lens(centre: Vector3<f64>, normal: Vector3<f64>, focal_length: f64, diameter: Option<f64>) -> OpticalElement {
        let mut plane = Plane::new(centre, normal);
        if let Some(d) = diameter {
            plane.with_radius(d)
        };
        let shape = Box::new(plane);
        let surface_properties = SurfaceProperties::new().thin_lens(focal_length).build().unwrap();
        OpticalElement {shape, surface_properties}
    }

    /// Returns `Some(Ray)` following the primary behaviour of the element's material, or None if
    /// the ray is blocked.
    /// 
    /// This is in constrast with `branching_ray` which may be split into multiple new rays.
    pub fn construction_ray(&self, input_ray: Ray, t: f64) -> Option<Ray> {
        use RayBehaviour::*;
        match self.surface_properties.primary_behaviour {
            Reflect => {
                let ray_intersection_position: Vector3<f64> = input_ray.position_at(t);
                let ray_reflected_direction: Vector3<f64> = reflect(input_ray.direction, self.shape.normal(ray_intersection_position));
                Some(Ray::new(ray_intersection_position, ray_reflected_direction, input_ray.n))
            },
            Absorb => None,
            Transmit => {
                let ray = self.surface_properties.refract(input_ray, t, &self.shape);
                Some(ray)
            },
        }
    }

    pub fn branching_ray(&self, _input_ray: Ray) -> Vec<Ray> {todo!()}
}