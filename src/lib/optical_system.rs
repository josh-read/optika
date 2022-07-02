use crate::*;
use nalgebra::Vector3;

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

    pub fn trace_ray(&self) {
        todo!()
    }
}
