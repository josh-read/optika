pub fn say_hello() {
    println!("Hello, world!");
}

pub mod ray;
pub mod shapes;
pub mod materials;
pub mod optical_element;
pub mod optical_system;

pub use ray::Ray;
pub use shapes::*;
pub use materials::SurfaceProperties;
pub use optical_element::OpticalElement;
pub use optical_system::OpticalSystem;