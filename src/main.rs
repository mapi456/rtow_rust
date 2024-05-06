use common::colour::Colour;
use common::interval::Interval;
use common::ray::Ray;
use common::vec3::{Point3, Vector3};
use engine::camera::Camera;
use primitive::hittable::{HitRecord, Hittable};
use primitive::hittable_list::HittableList;
use primitive::sphere::Sphere;

mod common;
mod primitive;
mod engine;


// WINDOW
const ASPECT_RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : i32 = 400;

// CAMERA
const FOCAL_LENGTH : f64 = 1.0;
const VIEWPORT_HEIGHT : f64 = 2.0;

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
    // Scene.
    let mut world = HittableList::new();

    (&mut world).add(Sphere::build_explicit((0.0, 0.0, -1.0), 0.5));
    (&mut world).add(Sphere::build_explicit((0.0, -100.5, -1.0), 100.0));

    // Camera.
    let mut camera = Camera::build(ASPECT_RATIO, IMAGE_WIDTH, FOCAL_LENGTH, VIEWPORT_HEIGHT);
    camera.initialize();

    camera.render(&world);
}
