use crate::*;
use nalgebra::Vector3;
use std::fmt;

#[derive(Debug)]
struct ApertureStop {
    index: usize,
    half_angle: f64,
    position: Vector3<f64>,
}

impl ApertureStop {
    pub fn new(index: usize, half_angle: f64, position: Vector3<f64>) -> Self {
        ApertureStop { index, half_angle, position }
    }
}

#[derive(Debug)]
struct FieldStop {
    index: usize,
    field_of_view: f64,
}

#[derive(Debug)]
struct SystemProperties {
    aperture_stop: ApertureStop,
    field_stop: FieldStop,
    magnification: f64,
}

pub struct OpticalSystem {
    elements: Vec<OpticalElement>,
    axis: Ray,
    tol: f64,
    aperture_stop: Option<ApertureStop>,
}

impl OpticalSystem {
    pub fn new(elements: Vec<OpticalElement>, axis: Ray, tol: f64) -> Self {
        OpticalSystem {
            elements,
            axis,
            tol,
            aperture_stop: None,
        }
    }

    /// For a given ray return the index and distance to the closest element in the system. If the
    /// ray does not intersect any elements, return `None`.
    fn closest_element(&self, ray: &Ray) -> Option<(usize, f64)> {
        let mut closest_i: Option<usize> = None;
        let mut closest_t = f64::INFINITY;
        for (i, e) in self.elements.iter().enumerate() {
            let t = e.shape.intersection(&ray).unwrap_or(f64::INFINITY);
            if t < closest_t {
                closest_i = Some(i);
                closest_t = t;
            }
            closest_t = t.min(closest_t)
        }
        if let Some(i) = closest_i {
            Some((i, closest_t))
        } else {
            None
        }
    }

    /// Takes an input ray and returns the next ray from whichever element intersects the
    /// ray first. If the ray does not intersect any elements or is absorbed then return `None`.
    pub fn trace_construction_ray(&self, ray: Ray) -> Option<(usize, Ray)> {
        let (i, t) = self.closest_element(&ray)?;
        let closest_element = &self.elements[i];
        if let Some(ray) = closest_element.construction_ray(ray, t) {
            Some((i, ray))
        } else {
            None
        }
    }

    /// Records the intersected elements and spawned rays by an input ray
    fn log_path(&self, ray: Ray) -> (Vec<usize>, Vec<Ray>) {
        let mut indices: Vec<usize> = Vec::new();
        let mut rays: Vec<Ray> = Vec::new();
        let mut ray = ray;
        loop {
            if let Some((i, r)) = self.trace_construction_ray(ray) {
                indices.push(i);
                rays.push(r);
                ray = r;
            } else {
                break
            }
        }
        (indices, rays)
    }
}

impl fmt::Debug for OpticalSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Traceable system with {} elements:", self.elements.len()));
        for (i, e) in self.elements.iter().enumerate() {
            let z = e.shape.centre()[2];
            if let DielectricProperties::ThinLens(f) = e.surface_properties.dielectric_properties {
                output.push_str(&format!("\ni = {}, z = {}, f = {}", i, z, f));
            } else {
                output.push_str(&format!("\ni = {}, z = {}", i, z));
            };
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use super::*;

    fn setup() -> OpticalSystem {
        let lens_1 = OpticalElement::new_thin_lens(
            100.0 * FORWARD,
            BACKWARD,
            100.0,
            Some(50.0)
        );
        let lens_2 = OpticalElement::new_thin_lens(
            400.0 * FORWARD,
            BACKWARD,
            200.0,
            Some(50.0)
        );
        let lens_3 = OpticalElement::new_thin_lens(
            650.0 * FORWARD,
            BACKWARD,
            50.0,
            Some(25.0)
        );
        let lens_4 = OpticalElement::new_thin_lens(
            800.0 * FORWARD,
            BACKWARD,
            100.0,
            Some(50.0)
        );
        let elements: Vec<OpticalElement> = vec![lens_1, lens_2, lens_3, lens_4];
        OpticalSystem::new(
            elements,
            AXIAL,
            1e-6
        )
    }

    #[test]
    fn closest() {
        let os = setup();
        let res = os.closest_element(&AXIAL).unwrap();
        assert_eq!(res, (0, 100.0));
    }

    #[test]
    fn trace_construction() {
        let os = setup();
        let res = os.trace_construction_ray(AXIAL).unwrap();
        assert_eq!(res, (0, Ray::new(100.0 * FORWARD, FORWARD, 1.0)));
    }
}