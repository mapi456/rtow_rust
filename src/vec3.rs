use core::panic;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug)]
#[derive(Clone)]
pub struct Vector3 {
    data: [f64; 3]
}

pub type Point3 = Vector3;

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 {
            data: [0.0, 0.0, 0.0]
        }
    }

    pub fn build(x: f64, y: f64, z: f64) -> Vector3{
        Vector3 {
            data: [x, y, z]
        }
    }

    pub fn x(& self) -> f64 {
        self.data[0]
    }
    
    pub fn y(& self) -> f64 {
        self.data[1]
    }
     
    pub fn z(& self) -> f64 {
        self.data[2]
    }

    pub fn length_squared(& self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn length(& self) -> f64 {
        f64::sqrt(self.length_squared())
    }

}

// impl Clone for Vector3 {
//     fn clone(&self) -> Self {
//         Self {
//             data: [self.x(), self.y(), self.z()]
//         }
//     }
// }

// Index Operators

impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            1..=3 => self.data.get(index)
                .expect("Vector3 array length too small."),
            _ => panic!()
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            1..=3 => self.data.get_mut(index)
                .expect("Vector3 array length too small."),
            _ => panic!()
        }
    }
}


// Subtraction Operators

impl Sub for & Vector3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        Vector3 {
            data: [self.x() - other.x(), self.y() - other.y(), self.z() - self.z()]
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            data: [self.x() - other.x(), self.y() - other.y(), self.z() - self.z()]
        }
    }
}

impl SubAssign<& Vector3> for Vector3 {
    fn sub_assign(&mut self, other: & Vector3) {
        self.data[0] -= other.x();
        self.data[1] -= other.y();
        self.data[2] -= other.z();
    }
}

impl Neg for Vector3 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            data: [-self.x(), -self.y(), -self.z()]
        }
    }
}


// Addition Operators

impl Add for & Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Vector3 {
        Vector3 {
            data: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl AddAssign<& Vector3> for Vector3 {
    fn add_assign(&mut self, other: & Vector3) {
        self.data[0] += other.x();
        self.data[1] += other.y();
        self.data[2] += other.z();
    }
}

// Multiplication Operators

impl Mul for & Vector3 {
    type Output = Vector3;

    fn mul(self, other: Self) -> Vector3 {
        Vector3 {
            data: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()]
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()]
        }
    }
}

impl Mul<& f64> for & Vector3 {
    type Output = Vector3;

    fn mul(self, other: & f64) -> Vector3 {
        Vector3 {
            data: [self.x() * other, self.y() * other, self.z() * other]
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            data: [self.x() * other, self.y() * other, self.z() * other]
        }
    }
}

impl Mul<& Vector3> for & f64 {
    type Output = Vector3;

    fn mul(self, other: & Vector3) -> Vector3 {
        Vector3 {
            data: [other.x() * self, other.y() * self, other.z() * self]
        }
    }
}

impl MulAssign<& Vector3> for Vector3 {
    fn mul_assign(&mut self, other: & Vector3) {
        self.data[0] *= other.x();
        self.data[1] *= other.y();
        self.data[2] *= other.z();
    }
}

impl MulAssign<& f64> for Vector3 {
    fn mul_assign(&mut self, other: & f64) {
        self.data[0] *= other;
        self.data[1] *= other;
        self.data[2] *= other;
    }
}



// Division Operator

impl Div<& f64> for & Vector3 {
    type Output = Vector3;

    fn div(self, other: & f64) -> Vector3 {
        self * &(1.0 / other)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        &self * &(1.0 / other)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        *self *= &(1.0 / other);
    }
}

// Vector Operators

pub fn unit_vector(vector: & Vector3) -> Vector3 {
    vector / &vector.length()
}

pub fn cross_product(vector1: & Vector3, vector2: & Vector3) -> Vector3 {
    Vector3 {
        data: [
            vector1.y() * vector2.z() - vector1.z() * vector2.y(),
            vector1.z() * vector2.x() - vector1.x() * vector2.z(),
            vector1.x() * vector2.y() - vector1.y() * vector2.x(),
        ]
    }
}

pub fn dot_product(vector1: & Vector3, vector2: & Vector3) -> f64 {
    vector1.x() * vector2.x() + vector1.y() * vector2.y() + vector1.z() * vector2.z()
}