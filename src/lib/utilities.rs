use approx::ulps_eq;
use crate::constants::*;
use nalgebra::Vector3;

/// For a given ray, return the default basis vectors, e.g. the vertical axis points UP while the
/// horizontal axis points to the right. In the edge case where the ray is pointing straight UP or
/// DOWN, the vertical axis is oriented along the FORWARD axis.
pub fn basis_vectors(axial_direction: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    debug_assert!(ulps_eq!(axial_direction.norm(), 1.0));
    let (right, up) = if axial_direction == &UP {
        // camera below subject
        (LEFT, FORWARD)
    } else if axial_direction == &DOWN {
        // camera above subject
        (RIGHT, FORWARD)
    } else {
        let mut right = axial_direction.cross(&UP);
        right.normalize_mut();
        let up = -1.0 * axial_direction.cross(&right);
        (right, up)
    };
    (right, up)
}