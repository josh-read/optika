use nalgebra::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub(crate) n: f64, // track the medium that the ray is currently in
}

impl Ray {
    /// Constructs a new `ray`.
    /// 
    /// The `origin` and `direction` must not contain any `f64::NAN` or `f64::INF` values, and
    /// `direction` must have a norm of 1. The refractive index `n` refers to the optical density
    /// at the ray's `origin` and must be greater than 1.
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, n: f64) -> Ray {
        Ray { origin, direction, n }
    }

    pub fn with_angle(&self, angle: f64, meridional_axis: Vector3<f64>) -> Ray {
        let direction = {
            let parallel = angle.cos() * self.direction;
            let perpendicular = angle.sin() * meridional_axis;
            parallel + perpendicular
        };
        Ray::new(self.origin, direction, self.n)
    }

    /// Returns the ray's position after it has propagated a distance `t`.
    pub fn position_at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}