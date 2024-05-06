use crate::common::vec3::{unit_vector, Point3, Vector3};
use crate::common::colour::{Colour, write_colour};
use crate::common::interval::Interval;
use crate::common::ray::Ray;

use crate::primitive::hittable::Hittable;

const PPM_FORMAT : &str = "P3\n";
const PPM_MAX_COLOUR : i32 = 255;

fn ray_colour(ray: & Ray, world: & impl Hittable) -> Colour {
    let (hit_anything, hit_rec) = world.hit(ray, &Interval::build(0.0, f64::INFINITY));

    if hit_anything {
        let normal = hit_rec.expect("camera::ray_colour: hit registered, but no hit record.").normal();
        return (normal + Colour::build(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = unit_vector(ray.direction());
    let a = (&unit_direction.y() + 1.0) * 0.5;
    Colour::build(1.0, 1.0, 1.0) * (1.0 - a) + Colour::build(0.5, 0.7, 1.0) * (a)
}

fn print_ppm_header(ppm_format : &str, image_height : i32, image_width : i32, ppm_max_colour : i32) {
    println!("{0}{1} {2}\n{3}", ppm_format, image_width, image_height, ppm_max_colour);
}

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,

    focal_length: f64,
    viewport_height: f64,

    image_height: Option<i32>,
    center: Option<Point3>,
    pixel00_loc: Option<Point3>,
    pixel_delta_width: Option<Vector3>,
    pixel_delta_height: Option<Vector3>,

    initialized: bool
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            focal_length: 1.0,
            viewport_height: 2.0,

            image_height: None,
            center: None,
            pixel00_loc: None,
            pixel_delta_width: None,
            pixel_delta_height: None,

            initialized: false
        }
    }

    pub fn build(aspect_ratio: f64, image_width: i32, focal_length: f64, viewport_height: f64) -> Camera {
        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            focal_length: focal_length,
            viewport_height: viewport_height,

            image_height: None,
            center: None,
            pixel00_loc: None,
            pixel_delta_width: None,
            pixel_delta_height: None,
            
            initialized: false
        }
    }

    // This is to facilitate initialize()...
    // but initialize() code should really be part of construction, or at least in Rust.
    fn image_height(&self) -> i32 {
        self.image_height.clone().expect("Camera: image_height needed, but not initialized.")
    }

    fn center(&self) -> Point3 {
        self.center.clone().expect("Camera: center needed, but not initialized.")
    }
    
    fn pixel00_loc(&self) -> Point3 {
        self.pixel00_loc.clone().expect("Camera: pixel00_loc needed, but not initialized.")
    }
    
    fn pixel_delta_width(&self) -> Vector3 {
        self.pixel_delta_width.clone().expect("Camera: pixel_delta_width needed, but not initialized.")
    }
    
    fn pixel_delta_height(&self) -> Vector3 {
        self.pixel_delta_height.clone().expect("Camera: pixel_delta_height needed, but not initialized.")
    }
    

    pub fn initialize(&mut self) {
        let mut projected_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if projected_height < 1 { 
            self.image_height = Some(1);
            projected_height = 1;
        } else { 
            self.image_height = Some(projected_height);
        };

        self.center = Some(Point3::new());

        // Viewport.
        let viewport_width = self.viewport_height * (self.image_width as f64 / projected_height as f64);

        // Viewport edge vectors.
        let viewport_width_vector = Vector3::build(viewport_width, 0.0, 0.0);
        let viewport_height_vector = Vector3::build(0.0, -self.viewport_height, 0.0);

        // Viewport pixel delta vectors.
        self.pixel_delta_width = Some( &viewport_width_vector / &(self.image_width as f64) );
        self.pixel_delta_height = Some( &viewport_height_vector / &(self.image_height() as f64) );

        // Pixel (0, 0) location.
        let viewport_upper_left = 
            self.center() 
                - Vector3::build(0.0, 0.0, self.focal_length)
                - viewport_width_vector / 2.0
                - viewport_height_vector / 2.0;
        self.pixel00_loc = Some(viewport_upper_left + (self.pixel_delta_width() + self.pixel_delta_height()) * 0.5);

        self.initialized = true;
    }

    pub fn render(&self, world: & impl Hittable) {
        if !self.initialized {
            eprintln!("Camera: render attempted without initialization.");
            panic!();
        }

        // Rendering.
        print_ppm_header(PPM_FORMAT, self.image_height(), self.image_width, PPM_MAX_COLOUR);
        
        for y in 0..self.image_height() {
            eprintln!("Scanlines remaining: {}", self.image_height() - y);

            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc() + (self.pixel_delta_width() * x as f64) + (self.pixel_delta_height() * (y as f64));
                let ray_direction = pixel_center - self.center();

                let r = Ray::build(&self.center(), &ray_direction);

                let pixel_colour = ray_colour(&r, world);
                write_colour(&pixel_colour);
            }
        }
    }
}