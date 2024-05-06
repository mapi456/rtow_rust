use super::vec3::Vector3;
use super::vec3::Point3;

pub struct Ray {
    origin: Point3,
    direction: Vector3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            direction: Vector3::new()
        }
    }

    pub fn build(origin: & Point3, direction: & Vector3) -> Ray {
        Ray {
            origin: Point3::clone(origin),
            direction: Vector3::clone(direction)
        }
    }

    pub fn origin(& self) -> & Point3{
        return &self.origin
    }

    pub fn direction(& self) -> & Vector3 {
        return &self.direction
    }

    pub fn at(& self, t: & f64) -> Point3 {
        return &self.origin + &(&self.direction * t);
    }
}