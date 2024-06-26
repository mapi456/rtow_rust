
use std::rc::Rc;

use crate::common::interval::Interval; 
use crate::common::ray::Ray;
use crate::common::vec3::{ dot_product, Point3 };

use crate::materials::Material;

use super::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    material: Rc<Box<dyn Material>>,
    radius: f64
}

impl Sphere {
    pub fn new(material: & Rc<Box<dyn Material>>) -> Sphere {
        Sphere {
            center: Point3::new(),
            material: Rc::clone(material),
            radius: 1.0,
        }
    }

    pub fn build(center: & Point3, radius: & f64, material: & Rc<Box<dyn Material>>) -> Sphere{
        Sphere {
            center: Point3::clone(center),
            material: Rc::clone(material),
            radius: f64::clone(radius)
        }
    }
    
    pub fn build_explicit(center: (f64, f64, f64), radius: f64, material: & Rc<Box<dyn Material>>) -> Sphere{
        Sphere {
            center: Point3::build(center.0, center.1, center.2),
            material: Rc::clone(material),
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(& self, ray: & Ray, ray_t: & Interval) -> (bool, Option<HitRecord>) {
        let origin_to_center = &self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot_product(ray.direction(), &origin_to_center);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { return (false, None); }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root)
            {
                root = (h + sqrtd) / a; 
                if !ray_t.surrounds(root) {
                    return (false, None) 
                } 
            } 
            
        let p = ray.at(&root);
        let outward_normal = (&p - &self.center) / self.radius;
        
        let mut hit_record = HitRecord{
            t: root,
            normal: None, 
            material: Rc::clone(&self.material),
            point: p,
            front_face: None
        };

        hit_record.set_face_normal(&ray, &outward_normal);

        (true, Some(hit_record))
        
    }
}