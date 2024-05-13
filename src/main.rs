use std::f64::consts::PI;
use std::rc::Rc;

use common::colour::Colour;
use common::interval::Interval;
use common::random::{random_f64, random_f64_standard};
use common::ray::Ray;
use common::vec3::Point3;

use engine::camera::Camera;

use materials::dielectric::Dialectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::{MatRc, Material};

use primitive::hittable::Hittable;
use primitive::hittable_list::HittableList;
use primitive::sphere::Sphere;

mod common;
mod primitive;
mod engine;
mod materials;


// WINDOW
const ASPECT_RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : i32 = 1200;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

// CAMERA
const CAMERA_VERTICAL_FOV: f64 = 20.0;
const CAMERA_DEFOCUS_ANGLE: f64 = 0.6;
const CAMERA_FOCUS_DIST: f64 = 10.0;

// MATERIAL THRESHOLDS FOR FINAL SCENE
const DIFFUSE: f64 = 0.8;
const METAL: f64 = 0.95;

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
    let mut world = HittableList::new();

    let ground_material: MatRc  = Rc::new(Box::from(Lambertian::from(Colour::build(0.5, 0.5, 0.5))));
    world.add(Sphere::build_explicit((0.0, -1000.0, 0.0), 1000.0, &ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64_standard();
            let center = Point3::build(a as f64 + 0.9 * random_f64_standard(), 0.2, b as f64 + 0.9 * random_f64_standard());

            if (center - Point3::build(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < DIFFUSE {
                    let albedo = Colour::random_standard() * Colour::random_standard();
                    let sphere_material: MatRc = Rc::new(Box::from(Lambertian::from(albedo)));
                    world.add(Sphere::build(&center, &0.2, &sphere_material));
                } else if choose_mat < METAL {
                    let albedo = Colour::random(0.5, 1.0);
                    let fuzz = random_f64(0.0, 0.5);
                    let sphere_material: MatRc = Rc::new(Box::from(Metal::from(albedo, fuzz)));
                    world.add(Sphere::build(&center, &0.2, &sphere_material));
                } else {
                    let albedo = Colour::new();
                    let sphere_material: MatRc = Rc::new(Box::from(Dialectric::from(albedo, 1.5)));
                    world.add(Sphere::build(&center, &0.2, &sphere_material));
                }
            }
        }
    }

    let material_1: MatRc = Rc::new(Box::from(Dialectric::from(Colour::new(), 1.5)));
    world.add(Sphere::build_explicit((0.0, 1.0, 0.0), 1.0, &material_1));
    
    let material_2: MatRc = Rc::new(Box::from(Lambertian::from(Colour::build(0.4, 0.2, 0.1))));
    world.add(Sphere::build_explicit((-4.0, 1.0, 0.0), 1.0, &material_2));
    
    let material_3: MatRc = Rc::new(Box::from(Metal::from(Colour::build(0.7, 0.6, 0.5), 0.0)));
    world.add(Sphere::build_explicit((4.0, 1.0, 0.0), 1.0, &material_3));

    // Camera.
    let camera_lookfrom = Point3::build(13.0, 2.0, 3.0);
    let camera_lookat = Point3::build(0.0, 0.0, 0.0);
    let camera_vup = Point3::build(0.0, 1.0, 0.0);

    let mut camera = Camera::build(
        ASPECT_RATIO, 
        IMAGE_WIDTH, 
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        CAMERA_VERTICAL_FOV,
        camera_lookfrom,
        camera_lookat,
        camera_vup,
        CAMERA_DEFOCUS_ANGLE,
        CAMERA_FOCUS_DIST
    );

    camera.initialize();
    camera.render(&world);
}
