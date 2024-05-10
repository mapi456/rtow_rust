use std::f64::consts::PI;
use std::rc::Rc;

use common::colour::Colour;
use common::interval::Interval;
use common::ray::Ray;
use common::vec3::Point3;

use engine::camera::Camera;

use materials::dielectric::Dialectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::Material;

use primitive::hittable::Hittable;
use primitive::hittable_list::HittableList;
use primitive::sphere::Sphere;

mod common;
mod primitive;
mod engine;
mod materials;


// WINDOW
const ASPECT_RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : i32 = 400;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

// CAMERA
const CAMERA_VERTICAL_FOV: f64 = 20.0;

fn ray_colour(ray: & Ray, world: & dyn Hittable) -> Colour{
    let (hit_anything, hit_rec) = world.hit(ray, &Interval::build(0.0, f64::INFINITY));

    if hit_anything {
        let normal = hit_rec.expect("ray_colour: Hit reported, but hit record missing.")
                                .normal.expect("hit record normal missing when requested.");
        return (Colour::build(1.0, 1.0, 1.0) + normal) * 0.5;
    }

    let unit_direction = common::vec3::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Colour::build(1.0, 1.0, 1.0) * (1.0 - a) + Colour::build(0.5, 0.7, 1.0) * (a)
}

fn main() {
    let R: f64 = f64::cos(PI / 4.0);

    // // Materials.
    let material_ground: Rc<Box<dyn Material>> = Rc::new(Box::from(Lambertian::from(Colour::build(0.8, 0.8, 0.0))));
    let material_center: Rc<Box<dyn Material>> = Rc::new(Box::from(Lambertian::from(Colour::build(0.1, 0.2, 0.5))));
    let material_left: Rc<Box<dyn Material>> = Rc::new(Box::from(Dialectric::from(Colour::build(0.8, 0.8, 0.8), 1.50)));
    let material_bubble: Rc<Box<dyn Material>> = Rc::new(Box::from(Dialectric::from(Colour::build(0.8, 0.8, 0.8), 1.0 / 1.50)));
    let material_right: Rc<Box<dyn Material>> = Rc::new(Box::from(Metal::from(Colour::build(0.8, 0.6, 0.2), 1.0)));

    // Scene.
    let mut world = HittableList::new();
    
    world.add(Sphere::build_explicit((0.0, -100.5, -1.0), 100.0, &material_ground));
    world.add(Sphere::build_explicit((0.0, 0.0, -1.2), 0.5, &material_center));
    world.add(Sphere::build_explicit((-1.0, 0.0, -1.0), 0.5, &material_left));
    world.add(Sphere::build_explicit((-1.0, 0.0, -1.0), 0.4, &material_bubble));
    world.add(Sphere::build_explicit((1.0, 0.0, -1.0), 0.5, &material_right));

    // Camera.
    let camera_lookfrom = Point3::build(-2.0, 2.0, 1.0);
    let camera_lookat = Point3::build(0.0, 0.0, -1.0);
    let camera_vup = Point3::build(0.0, 1.0, 0.0);

    let mut camera = Camera::build(
        ASPECT_RATIO, 
        IMAGE_WIDTH, 
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        CAMERA_VERTICAL_FOV,
        camera_lookfrom,
        camera_lookat,
        camera_vup
    );

    camera.initialize();
    camera.render(&world);
}
