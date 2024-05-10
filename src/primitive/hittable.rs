use std::rc::Rc;

use crate::common::interval::Interval; 
use crate::common::ray::Ray;
use crate::common::vec3::{ dot_product, Point3, Vector3 };

use crate::materials::Material;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Option<Vector3>,
    pub material: Rc<Box<dyn Material>>,
    pub t: f64,
    pub front_face: Option<bool>
}

impl HitRecord {
    pub fn normal(&self) -> Vector3 {
        self.normal.clone().expect("HitRecord: normal requested, but no normal found.")
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn material<'a>(&self) -> Rc<Box<dyn Material>>{
        Rc::clone(&self.material)
    }

    pub fn front_face(&self) -> bool {
        self.front_face.clone().expect("HitRecord; front_face requested, but not found.")
    }

    pub fn set_face_normal(&mut self, ray: & Ray, outward_normal: & Vector3) {
        // TODO: assert that outward_normal is unit length (within margin)

        let front = dot_product(ray.direction(), outward_normal) < 0.0;
        if front {
            self.normal = Some(outward_normal.clone());
        } else {
            self.normal = Some(-outward_normal);
        }
        self.front_face = Some(front);
    }
}

pub trait Hittable {
    fn hit(& self, ray: & Ray, ray_t: & Interval) -> (bool, Option<HitRecord>);
}