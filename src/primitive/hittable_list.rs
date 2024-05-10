use std::cell::RefCell;
use std::vec;

use crate::common::interval::Interval;

use super::hittable::{ HitRecord, Hittable };

pub struct HittableList<'a> {
    objects: RefCell<Vec<Box<dyn Hittable + 'a>>>
}

impl HittableList<'_> {
    pub fn clear(&mut self) {
        self.objects.get_mut().clear();
    }
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList {
            objects: RefCell::new(Vec::new())
        }
    }

    pub fn build(object: impl Hittable + 'a) -> HittableList<'a> {
        HittableList { objects: RefCell::new(vec![Box::new(object)]) }
    }

    pub fn add(&self, object: impl Hittable + 'a) {
        self.objects.try_borrow_mut().expect("help").push(Box::new(object));
    }
}   

impl Hittable for HittableList<'_> {
    fn hit(& self, ray: & crate::common::ray::Ray, ray_t: & Interval) -> (bool, Option<super::hittable::HitRecord>) {
        let mut hit_rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_t = ray_t.max();

        let objects = self.objects.try_borrow_mut().expect("help");

        for object in objects.iter() {
            let ptr = object;
            let (hit, temp_rec) = ptr.hit(ray, &Interval::build(ray_t.min(), closest_t));
            if hit { 
                let rec_val = temp_rec.expect("HittableList.hit(): hit registered, but no record available");
                closest_t = (&rec_val).t.clone();
                hit_rec = Some(rec_val);
                hit_anything = true;
            }
        }

        (hit_anything, hit_rec)
    }
}