use crate::common::colour::Colour;
use crate::common::ray::Ray;
use crate::common::vec3::{ dot_product, reflect, refract, unit_vector };

use crate::primitive::hittable::HitRecord;

use super::Material;

pub struct Dialectric {
    albedo: Colour,
    refractive_index: f64
}


impl Dialectric {
    pub fn new() -> Dialectric {
        Dialectric {
            albedo: Colour::build(0.5, 0.5, 0.5),
            refractive_index: 0.5
        }
    }

    pub fn build(colour: Colour, refractive_index: f64) -> Dialectric {
        Dialectric {
            albedo: colour.clone(),
            refractive_index
        }
    }

    pub fn build_explicit(r: f64, g: f64, b: f64, refractive_index: f64) -> Dialectric {
        Dialectric {
            albedo: Colour::build(r, g, b),
            refractive_index
        }
    }

    pub fn from(colour: Colour, refractive_index: f64) -> Dialectric {
        Dialectric {
            albedo: colour,
            refractive_index
        }
    }
}


impl Material for Dialectric {
    fn scatter(
            &self,
            ray_in: & Ray,
            hit_rec: & HitRecord
        ) -> (bool, Colour, Ray) {
        let attenuation = Colour::build(1.0, 1.0, 1.0);
        let normal = &hit_rec.normal();

        let refractive_index_ratio = 
            if hit_rec.front_face() { 1.0 / self.refractive_index } else { self.refractive_index / 1.0 };

        let unit_direction = unit_vector(ray_in.direction());

        let cos_theta = dot_product(&-unit_direction, &hit_rec.normal()).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refractive_index_ratio * sin_theta > 1.0;
        let direction = 
            if cannot_refract { reflect(&unit_direction, &normal) } else { refract(&unit_direction, &normal, refractive_index_ratio) };

        (true, attenuation, Ray::from(hit_rec.point(), direction))
    }
}