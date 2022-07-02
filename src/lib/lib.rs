pub fn say_hello() {
    println!("Hello, world!");
}

pub mod ray;
pub mod shapes;

pub use ray::*;
pub use shapes::*;