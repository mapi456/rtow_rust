
use crate::common::{ray::Ray, vec3::{dot_product, Point3}};

use super::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Point3::new(),
            radius: 1.0,
        }
    }

    pub fn build(center: & Point3, radius: & f64) -> Sphere{
        Sphere {
            center: Point3::clone(center),
            radius: f64::clone(radius)
        }
    }
    
    pub fn build_explicit(center: (f64, f64, f64), radius: f64) -> Sphere{
        Sphere {
            center: Point3::build(center.0, center.1, center.2),
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(& self, ray: & Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let origin_to_center = &self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot_product(ray.direction(), &origin_to_center);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 { return (false, None); }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;

        if root <= t_min || root >= t_max 
            {
                root = (h + sqrtd) / a; 
                if root <= t_min || root >= t_max {
                    return (false, None) 
                } 
            } 
            
        let p = ray.at(&root);
        let outward_normal = (&p - &self.center) / self.radius;

        if dot_product(ray.direction(), &outward_normal) > 0.0 {
            eprintln!("Backface detected!");
            eprintln!("This ray -> origin: {:?}, direction: {:?}", ray.origin(), ray.direction());
            eprintln!("Intersection was on the sphere at point {:?}", p);
            let root1 = (h - sqrtd) / a;
            let root2 = (h + sqrtd) / a;
            eprintln!("first root was: {}, gives ray.at(t): {:?}", (h - sqrtd) / a, ray.at(&root1));
            eprintln!("first root was: {}, gives ray.at(t): {:?}", (h + sqrtd) / a, ray.at(&root2));
            
            
        }
        
        let mut hit_record = HitRecord{
            t: root,
            normal: None, 
            point: p, 
            front_face: None
        };

        hit_record.set_face_normal(&ray, &outward_normal);

        (true, Some(hit_record))
        
    }
}