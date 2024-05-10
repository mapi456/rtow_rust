use crate::common::mat3::Matrix3;
use crate::common::random::{ sample, Distributions };
use crate::common::vec3::{ cross_product, unit_vector, Point3, Vector3 };
use crate::common::colour::{ Colour, write_colour };
use crate::common::interval::Interval;
use crate::common::ray::Ray;
use crate::common::{ degrees_to_radians, RAY_MINIMUM_DISTANCE_BEFORE_HIT };

use crate::primitive::hittable::Hittable;

const PPM_FORMAT : &str = "P3\n";
const PPM_MAX_COLOUR : i32 = 255;


// Raycast helpers.

fn ray_colour(ray: & Ray, depth_remaining: i32, world: & impl Hittable) -> Colour {
    if depth_remaining <= 0 {
        return Colour::new()
    }

    let (hit_anything, hit_rec) = 
        world.hit(ray, &Interval::build(RAY_MINIMUM_DISTANCE_BEFORE_HIT, f64::INFINITY));

    if hit_anything {
        let hit_rec = hit_rec.expect("camera::ray_colour: hit registered, but no hit record.");

        let (scattered, attenuation, scattered_ray) = 
            hit_rec.material.scatter(ray, &hit_rec);

        if scattered {
            return attenuation * ray_colour(&scattered_ray, depth_remaining - 1, world);
        }

        return Colour::build(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(ray.direction());
    let a = (&unit_direction.y() + 1.0) * 0.5;
    Colour::build(1.0, 1.0, 1.0) * (1.0 - a) + Colour::build(0.5, 0.7, 1.0) * (a)
}

// I think this is a function mostly to demonstrate that the pixel shape from
// which samples are drawn can be switched to produce different types of images.
// Should probably be partitioned into random, then, or something else.
fn sample_square() -> Vector3 {
    // Produces a random vector within the [-0.5, -0.5, 0.0] - [0.5, 0.5, 0.0] unit square.
    Vector3::build(
        sample(Distributions::UNIFORM_INCL(-0.5, 0.5)), 
        sample(Distributions::UNIFORM_INCL(-0.5, 0.5)), 
        0.0
    )
}

// Output helper.

fn print_ppm_header(ppm_format : &str, image_height : i32, image_width : i32, ppm_max_colour : i32) {
    println!("{0}{1} {2}\n{3}", ppm_format, image_width, image_height, ppm_max_colour);
}

pub struct Camera {
    // configurable attributes
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,

    vertical_fov_degrees: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vector3,

    // computed attributes
    image_height: Option<i32>,
    viewport_height: Option<f64>,
    pixel_samples_scale: Option<f64>,
    center: Option<Point3>,
    pixel00_loc: Option<Point3>,
    pixel_delta_width: Option<Vector3>,
    pixel_delta_height: Option<Vector3>,
    frame_basis: Option<Matrix3>,

    initialized: bool
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vertical_fov_degrees: 90.0,
            lookfrom: Point3::build(0.0, 0.0, 0.0),
            lookat: Point3::build(0.0, 0.0, -1.0),
            vup: Vector3::build(0.0, -1.0, 0.0),

            image_height: None,
            viewport_height: None,
            pixel_samples_scale: None,
            center: None,
            pixel00_loc: None,
            pixel_delta_width: None,
            pixel_delta_height: None,
            frame_basis: None,

            initialized: false
        }
    }

    pub fn build(
        aspect_ratio: f64, 
        image_width: i32, 
        samples_per_pixel: i32, 
        max_depth: i32, 
        vertical_fov_degrees: f64, 
        lookfrom: Point3,
        lookat: Point3,
        vup: Vector3
    ) -> Camera {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vertical_fov_degrees,
            lookfrom,
            lookat,
            vup,

            image_height: None,
            viewport_height: None,
            pixel_samples_scale: None,
            center: None,
            pixel00_loc: None,
            pixel_delta_width: None,
            pixel_delta_height: None,
            frame_basis: None,
            
            initialized: false
        }
    }

    // This is to facilitate initialize()...
    // but initialize() code should really be part of construction, or at least in Rust.
    fn image_height(&self) -> i32 {
        self.image_height.clone().expect("Camera: image_height needed, but not initialized.")
    }

    fn pixel_samples_scale(&self) -> f64 {
        self.pixel_samples_scale.clone().expect("Camera: pixel_samples_scale needed, but not initialized.")
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

    fn frame_basis(&self) -> Matrix3 {
        self.frame_basis.clone().expect("Camera: frame_basis needed, but not initialized")
    }
    
    fn ray_to_pixel(&self, x: i32, y: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = 
            self.pixel00_loc() + 
            (self.pixel_delta_width() * (x as f64 + offset.x()))+ 
            (self.pixel_delta_height() * (y as f64 + offset.y()));

        let ray_origin = self.center();
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    pub fn initialize(&mut self) {
        let mut projected_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if projected_height < 1 { 
            self.image_height = Some(1);
            projected_height = 1;
        } else { 
            self.image_height = Some(projected_height);
        };

        self.pixel_samples_scale = Some(1.0 / self.samples_per_pixel as f64);

        self.center = Some(self.lookfrom);

        // Viewport.
        let focal_length = (self.lookat - self.lookfrom).length();
        let theta = degrees_to_radians(self.vertical_fov_degrees);
        let h = (theta / 2.0).tan();
        self.viewport_height = Some(2.0 * h * focal_length);
        let viewport_width = self.viewport_height.expect("Camera: viewport height was just set but no longer exists.") * (self.image_width as f64 / projected_height as f64);

        // Calculate camera coordinate frame basis vectors.
        let w = unit_vector(&(self.lookfrom - self.lookat));
        let u = unit_vector(&cross_product(&self.vup, &w));
        self.frame_basis = Some(Matrix3::build(&u, &cross_product(&w, &u), &w));

        // Viewport edge vectors.
        let viewport_width_vector = u * viewport_width;
        let viewport_height_vector = -self.frame_basis().v() * self.viewport_height.expect("msg");

        // Viewport pixel delta vectors.
        self.pixel_delta_width = Some( &viewport_width_vector / &(self.image_width as f64) );
        self.pixel_delta_height = Some( &viewport_height_vector / &(self.image_height() as f64) );

        // Pixel (0, 0) location.
        let viewport_upper_left = 
            self.center() 
                - w * focal_length
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
        
        let spp = self.pixel_samples_scale();
        for y in 0..self.image_height() {
            eprintln!("Scanlines remaining: {}", self.image_height() - y);

            for x in 0..self.image_width {
                let mut pixel_colour = Colour::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.ray_to_pixel(x, y);
                    pixel_colour += ray_colour(&r, self.max_depth, world);
                }
                write_colour(&(pixel_colour * spp));
            }
        }
    }
}