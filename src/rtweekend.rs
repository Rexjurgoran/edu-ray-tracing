use rand::prelude::*;

// Constants
const PI: f64 = 3.1415926535897932385;

// Utility Functions
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_double_from(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
