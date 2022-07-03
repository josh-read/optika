use crate::ray::Ray;
use nalgebra::{vector, Vector3};

pub const ORIGIN: Vector3<f64> = vector![0.0, 0.0, 0.0];
pub const UP: Vector3<f64> = vector![0.0, 1.0, 0.0];
pub const DOWN: Vector3<f64> = vector![0.0, -1.0, 0.0];
pub const LEFT: Vector3<f64> = vector![-1.0, 0.0, 0.0];
pub const RIGHT: Vector3<f64> = vector![1.0, 0.0, 0.0];
pub const FORWARD: Vector3<f64> = vector![0.0, 0.0, -1.0];
pub const BACKWARD: Vector3<f64> = vector![0.0, 0.0, 1.0];

pub const AXIAL: Ray = Ray {
    origin: ORIGIN,
    direction: FORWARD,
    n: 1.0,
};