use crate::*;
use std::f64::consts::PI;
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
        while let Some((i, r)) = self.trace_construction_ray(ray) {
            indices.push(i);
            rays.push(r);
            ray = r;
        }
        (indices, rays)
    }

    pub fn numerical_aperture(&self) {
        // calculate basis vectors of optical system
        let (_sagittal_axis, meridional_axis) = basis_vectors(&self.axis.direction);
        // log the axial ray path
        let (axial_path, _) = self.log_path(self.axis);
        // find largest angle that follows same path
        let mut test_angles = [0.0, (PI / 4.0), (PI / 2.0)];
        let mut n_surfaces_before_block = 0;
        while (test_angles[2] - test_angles[0]).abs() > self.tol {
            // construct a ray with the test angle
            let test_ray = self.axis.with_angle(test_angles[1], meridional_axis);
            // test ray by tracing sequentially through the system
            let (test_path, _) = self.log_path(test_ray);
            if test_path == axial_path {
                test_angles[0] = test_angles[1];
            } else {
                n_surfaces_before_block = test_path.len();
                test_angles[2] = test_angles[1]
            }
            // select next test angle
            test_angles[1] = (test_angles[0] + test_angles[2]) / 2.0;
        }
        // follow ray through to marginal surface
        let marginal_ray = self.axis.with_angle(test_angles[0], meridional_axis);
        let blocked_ray = {
            let mut ray = marginal_ray;
            for _ in 0..n_surfaces_before_block {
                ray = self.trace_construction_ray(ray).unwrap().1;
            }
            ray
        };
        let aperture_stop_position = blocked_ray.origin;
        println!("{:?}", aperture_stop_position);
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
        let lens_2 = OpticalElement::new_thin_lens(
            100.0 * FORWARD,
            BACKWARD,
            100.0,
            Some(50.0)
        );
        let lens_1 = OpticalElement::new_thin_lens(
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
        assert_eq!(res, (1, 100.0));
    }

    #[test]
    fn trace_construction() {
        let os = setup();
        let res = os.trace_construction_ray(AXIAL);
        assert_eq!(res, Some((1, Ray::new(100.0 * FORWARD, FORWARD, 1.0))));
    }

    #[test]
    fn trace_construction_should_fail() {
        let os = setup();
        let res = os.trace_construction_ray(-AXIAL);
        assert_eq!(res, None);
    }
}