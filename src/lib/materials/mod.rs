use approx::ulps_eq;
use nalgebra::Vector3;

/// Refracted ray direction for incident ray refracting at an interface 
/// 
/// Note: for air to glass n1_over_n2 is 1/n_glass.
pub fn refract(v: Vector3<f64>, n: Vector3<f64>, n1_over_n2: f64) -> Vector3<f64> {
    debug_assert!(ulps_eq!(v.normalize(), v));
    debug_assert!(ulps_eq!(v.normalize(), v));

    let n = if v.dot(&n) > 0.0 {
        -n
    } else {
        n
    };
    let cos_theta_i = -v.dot(&n);

    n1_over_n2 * v
        + (n1_over_n2 * cos_theta_i
            - (1.0 - n1_over_n2 * n1_over_n2 * (1.0 - cos_theta_i * cos_theta_i)).sqrt())
            * n
}

/// Reflected ray direction for incident ray v reflecting from a surface with normal n.
pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    debug_assert!(ulps_eq!(v.normalize(), v));
    debug_assert!(ulps_eq!(n.normalize(), n));
    v - 2.0 * (v.dot(&n)) * n
}

/// Schlick approximation for reflectance
fn reflectance(i_r: f64, cos_theta: f64) -> f64 {
    let r_0 = ((1.0 - i_r) / (1.0 + i_r)).powi(2);
    r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5)
}

pub mod surface_properties;
pub mod surface_properties_builder;

pub use surface_properties::*;
pub use surface_properties_builder::*;