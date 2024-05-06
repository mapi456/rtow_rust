use std::vec;

use super::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn build(object: impl Hittable + 'static) -> HittableList {
        HittableList { objects: vec![Box::new(object)] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}   

impl Hittable for HittableList {
    fn hit(& self, ray: & crate::common::ray::Ray, t_min: f64, t_max: f64) -> (bool, Option<super::hittable::HitRecord>) {
        let mut hit_rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_t = t_max;

        for object in self.objects.iter() {
            let (hit, temp_rec) = object.hit(ray, t_min, closest_t);
            if hit { 
                let rec_val = temp_rec.expect("HittableList.hit(): hit registered, but no record available");
                closest_t = (&rec_val).t;
                hit_rec = Some(rec_val);
                hit_anything = true;
            }
        }

        (hit_anything, hit_rec)
    }
}