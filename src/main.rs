use colour::Colour;
use ray::Ray;
use vec3::{unit_vector, Point3, Vector3};

mod vec3;
mod colour;
mod ray;

const ASPECT_RATIO : f64 = 16.0 / 9.0;

// WINDOW
const IMAGE_WIDTH : i32 = 400;
const IMAGE_HEIGHT : i32 = if IMAGE_WIDTH as f64 > ASPECT_RATIO { (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32 } else { 1 } ;

// CAMERA
const FOCAL_LENGTH : f64 = 1.0;
const VIEWPORT_HEIGHT : f64 = 2.0;
const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const PPM_FORMAT : &str = "P3\n";
const PPM_MAX_COLOUR : i32 = 255;

const TO_COLOUR : f64 = 255.999;

fn print_ppm_header(ppm_format : &str, image_height : i32, image_width : i32, ppm_max_colour : i32) {
    println!("{0}{1} {2}\n{3}", ppm_format, image_height, image_width, ppm_max_colour);
}

fn ray_colour(ray: & Ray) -> Colour{
    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Colour::build(1.0, 1.0, 1.0) * (1.0 - a) + Colour::build(0.5, 0.7, 1.0) * (a)
}

fn main() {
    // Finish setting up engine.
    let camera_center : Point3 = Point3::build(0.0, 0.0, 0.0);
    
    let viewport_width_vector = Vector3::build(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_height_vector = Vector3::build(0.0, VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_width = &viewport_width_vector / &(IMAGE_WIDTH as f64);
    let pixel_delta_height = &viewport_height_vector / &(IMAGE_WIDTH as f64);

    let viewport_upper_left: Vector3 = 
        &camera_center 
            - &Vector3::build(0.0, 0.0, FOCAL_LENGTH) 
            - viewport_width_vector / 2.0
            - viewport_height_vector / 2.0;
    let pixel00_loc = viewport_upper_left + (&pixel_delta_width + &pixel_delta_height) * 0.5;
    

    // Rendering.
    print_ppm_header(PPM_FORMAT, IMAGE_HEIGHT, IMAGE_WIDTH, PPM_MAX_COLOUR);

    for y in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - y);

        for x in 0..IMAGE_WIDTH {
            let pixel_center = &pixel00_loc + &(&pixel_delta_width * &(x as f64)) + (&pixel_delta_height * &(y as f64));
            let ray_direction = &pixel_center - &camera_center;

            let r = Ray::build(&pixel_center, &ray_direction);

            let pixel_colour = ray_colour(&r);
            colour::write_colour(&pixel_colour);
        }
    }

}
