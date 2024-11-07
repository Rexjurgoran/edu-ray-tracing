use rand::prelude::*;

// Constants
const PI: f64 = 3.1415926535897932385;
pub const REFRACTION_GLASS: f64 = 1.50;
pub const REFRACTION_WATER: f64 = 1.33;
pub const REFRACTION_AIR: f64 = 1.00;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_double_from(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
