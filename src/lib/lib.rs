pub fn say_hello() {
    println!("Hello, world!");
}

pub mod ray;
pub mod shapes;
pub mod materials;

pub use ray::*;
pub use shapes::*;