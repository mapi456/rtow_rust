use std::rc::Rc;

use crate::common::{ colour::Colour, ray::Ray }; 
use crate::primitive::hittable::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub type MatRc = Rc<Box<dyn Material>>;

pub trait Material {
    fn scatter(
        &self,
        ray_in: & Ray,
        hit_rec: & HitRecord
    ) -> (bool, Colour, Ray);
}