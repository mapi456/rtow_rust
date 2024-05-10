use crate::common::colour::Colour;
use crate::common::ray::Ray;
use crate::common::vec3::random_unit_vector;

use crate::primitive::hittable::HitRecord;

use super::Material;

pub struct Lambertian {
    albedo: Colour
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian {
            albedo: Colour::build(0.5, 0.5, 0.5)
        }
    }

    pub fn build(colour: Colour) -> Lambertian {
        Lambertian {
            albedo: colour.clone()
        }
    }

    pub fn build_explicit(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian {
            albedo: Colour::build(r, g, b)
        }
    }

    pub fn from(colour: Colour) -> Lambertian {
        Lambertian {
            albedo: colour
        }
    }
}

impl Material for Lambertian {
    fn scatter(
            &self,
            ray_in: & Ray,
            hit_rec: & HitRecord
        ) -> (bool, Colour, Ray) {
            let mut scatter_direction = hit_rec.normal() + random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = hit_rec.normal()
            }

            let scattered_ray = Ray::build(&hit_rec.point(), &scatter_direction);
            (true, self.albedo, scattered_ray)
    }
}