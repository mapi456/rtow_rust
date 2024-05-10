use crate::common::colour::Colour; 
use crate::common::ray::Ray;
use crate::common::vec3::{ dot_product, random_unit_vector, reflect, unit_vector }; 

use crate::primitive::hittable::HitRecord;

use super::Material;

pub struct Metal {
    albedo: Colour,
    fuzz: f64
}

impl Metal {
    pub fn new() -> Metal {
        Metal {
            albedo: Colour::build(0.5, 0.5, 0.5),
            fuzz: 0.5
        }
    }

    pub fn build(colour: Colour, fuzz: f64) -> Metal {
        Metal {
            albedo: colour.clone(),
            fuzz
        }
    }

    pub fn build_explicit(r: f64, g: f64, b: f64, fuzz: f64) -> Metal {
        Metal {
            albedo: Colour::build(r, g, b),
            fuzz
        }
    }

    pub fn from(colour: Colour, fuzz: f64) -> Metal {
        Metal {
            albedo: colour,
            fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(
            &self,
            ray_in: & Ray,
            hit_rec: & HitRecord
        ) -> (bool, Colour, Ray) {
            let reflect_direction = reflect(ray_in.direction(), &hit_rec.normal());
            // this is where we add the fuzz
            // i wonder the result of biasing the random vector's direction towards the reflectance direction?
            let reflected = unit_vector(&reflect_direction) + (random_unit_vector() * self.fuzz);
            let scattered_ray = Ray::build(&hit_rec.point(), &reflected);

            (
                dot_product(scattered_ray.direction(), &hit_rec.normal()) > 0.0, 
                self.albedo, 
                scattered_ray
            )
    }
}