use crate::TO_COLOUR;
use crate::vec3::Vector3;

pub type Colour = Vector3;

pub fn write_colour(pixel_colour: & Colour) {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();

    let rbyte = (TO_COLOUR * r) as i32;
    let gbyte = (TO_COLOUR * g) as i32;
    let bbyte = (TO_COLOUR * b) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
