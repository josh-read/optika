use optika::*;
use optika::constants::*;

fn main() {
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
    let elements: Vec<OpticalElement> = vec![lens_1, lens_2];
    let mut os = OpticalSystem::new(
        elements,
        AXIAL,
        1e-6
    );
    println!("{:?}", os);
    println!("{:?}", os.trace_construction_ray(AXIAL));
    os.numerical_aperture();
}