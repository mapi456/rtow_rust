use crate::common::interval::Interval;

use super::vec3::Vector3;

const TO_COLOUR : f64 = 255.999;

pub type Colour = Vector3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

pub fn write_colour(pixel_colour: & Colour) {
    // retrieve the RGB values.
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    // this is explicitly separate in tutorial,
    // but we can roll it up to the earlier block, avoiding mut.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // [0, 1] => [0, 255]
    let intensity = Interval::build(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
