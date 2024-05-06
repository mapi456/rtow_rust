use std::f64::consts::PI;

pub mod colour;
pub mod ray;
pub mod vec3;
pub mod interval;
pub mod random;

// Important constants.

pub const RAY_MINIMUM_DISTANCE_BEFORE_HIT: f64 = 0.001;


// Utility functions.

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}