use crate::common::{ray::Ray, vec3::{dot_product, Point3, Vector3}};

// static mut BACKFACES: i32 = 0;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Option<Vector3>,
    pub t: f64,
    pub front_face: Option<bool>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: & Ray, outward_normal: & Vector3) {
        // TODO: assert that outward_normal is unit length (within margin)

        let front = dot_product(ray.direction(), outward_normal) < 0.0;
        if front {
            self.normal = Some(outward_normal.clone());
        } else {
            unsafe {
                // BACKFACES += 1;
                // eprintln!("back face #{} has been detected, normal: {:?}", BACKFACES, -outward_normal);
                // eprintln!("ray direction: {:?}, outward_normal: {:?}, results in dot product: {}", ray.direction(), outward_normal, dot_product(ray.direction(), outward_normal));
            }
            self.normal = Some(-outward_normal);
        }
        self.front_face = Some(front);
    }
}

pub trait Hittable {
    fn hit(& self, ray: & Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>);
}