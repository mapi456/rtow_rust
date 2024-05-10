use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use super::vec3::{dot_product, Vector3};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Matrix3 {
    // My matrix is row-major.
    data: [Vector3 ; 3]
}

impl Matrix3 {
    // Constructors
    pub fn new() -> Matrix3 {
        Matrix3 {
            data: [
                Vector3::new(),
                Vector3::new(),
                Vector3::new()
            ]
        }
    }

    pub fn build(u: & Vector3, v: & Vector3, w: & Vector3) -> Matrix3 {
        Matrix3 {
            data: [
                u.clone(),
                v.clone(),
                w.clone()
            ]
        }
    }

    pub fn build_explicit(u: (f64, f64, f64), v: (f64, f64, f64), w: (f64, f64, f64)) -> Matrix3 {
        Matrix3 {
            data: [
                Vector3::build(u.0, u.1, u.2),
                Vector3::build(v.0, v.1, v.2),
                Vector3::build(w.0, w.1, w.2)
            ]
        }
    }

    pub fn from(u: Vector3, v: Vector3, w: Vector3) -> Matrix3 {
        Matrix3 {
            data: [ u, v, w ]
        }
    }


    // Struct functions.
    pub fn u(& self) -> & Vector3 {
        &self.data[0]
    }
    
    pub fn v(& self) -> & Vector3 {
        &self.data[1]
    }
     
    pub fn w(& self) -> & Vector3 {
        &self.data[2]
    }

}


// Index Operator
impl Index<usize> for Matrix3 {
    type Output = Vector3;

    fn index(&self, index: usize) -> & Vector3 {
        match index {
            0..=2 => self.data.get(index)
                .expect("Vector3 array length too small."),
            _ => panic!()
        }
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Vector3 {
        match index {
            0..=2 => self.data.get_mut(index)
                .expect("Vector3 array length too small."),
            _ => panic!()
        }
    }
}


// Subtraction Operators

impl Sub for & Matrix3 {
    type Output = Matrix3;

    fn sub(self, other: Self) -> Matrix3 {
        Matrix3 {
            data: [self.u() - other.u(), self.v() - other.v(), self.w() - other.w()]
        }
    }
}

impl Sub for Matrix3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            data: [self.u() - other.u(), self.v() - other.v(), self.w() - other.w()]
        }
    }
}

impl SubAssign<& Matrix3> for Matrix3 {
    fn sub_assign(&mut self, other: & Matrix3) {
        self.data[0] -= other.u();
        self.data[1] -= other.v();
        self.data[2] -= other.w();
    }
}

impl Neg for Matrix3 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            data: [-self.u(), -self.v(), -self.w()]
        }
    }
}

impl Neg for & Matrix3 {
    type Output = Matrix3;
    
    fn neg(self) -> Matrix3 {
        Matrix3 {
            data: [-self.u(), -self.v(), -self.w()]
        }
    }
}


// Addition Operators

impl Add for & Matrix3 {
    type Output = Matrix3;

    fn add(self, other: Self) -> Matrix3 {
        Matrix3 {
            data: [self.u() + other.u(), self.v() + other.v(), self.w() + other.w()]
        }
    }
}

impl Add for Matrix3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: [self.u() + other.u(), self.v() + other.v(), self.w() + other.w()]
        }
    }
}


impl AddAssign<Matrix3> for Matrix3 {
    fn add_assign(&mut self, other: Matrix3) {
        self.data[0] += other.u();
        self.data[1] += other.v();
        self.data[2] += other.w();
    }
}

impl AddAssign<& Matrix3> for Matrix3 {
    fn add_assign(&mut self, other: & Matrix3) {
        self.data[0] += other.u();
        self.data[1] += other.v();
        self.data[2] += other.w();
    }
}


// Multiplication Operators

// TODO: define matrix multiplication

// 3x3 * 3x1 = 3x1
impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::build(
            dot_product(self.u(), &rhs),
            dot_product(self.v(), &rhs),
            dot_product(self.w(), &rhs)
        )
    }
}

impl Mul<& Vector3> for & Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: & Vector3) -> Vector3 {
        self.u() * rhs + self.v() * rhs + self.w() * rhs
    }
}

// 1x3 * 3x3 => 1x3
impl Mul<Matrix3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix3) -> Vector3 {
        let x = self.x() * rhs[0][0] + self.y() * rhs[1][0] + self.z() * rhs[2][0];
        let y = self.x() * rhs[0][1] + self.y() * rhs[1][1] + self.z() * rhs[2][1];
        let z = self.x() * rhs[0][2] + self.y() * rhs[1][2] + self.z() * rhs[2][2];
        Vector3::build(x, y, z)
    }
}

impl Mul<& Matrix3> for & Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: & Matrix3) -> Vector3 {
        let x = self.x() * rhs[0][0] + self.y() * rhs[1][0] + self.z() * rhs[2][0];
        let y = self.x() * rhs[0][1] + self.y() * rhs[1][1] + self.z() * rhs[2][1];
        let z = self.x() * rhs[0][2] + self.y() * rhs[1][2] + self.z() * rhs[2][2];
        Vector3::build(x, y, z)
    }
}

impl Mul<f64> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f64) -> Matrix3 {
        Matrix3 {
            data: [
                self.u() * rhs,
                self.v() * rhs,
                self.w() * rhs
            ]
        }
    }
}

impl Mul<Matrix3> for f64 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            data: [
                self * rhs.u(),
                self * rhs.v(),
                self * rhs.w()
            ]
        }
    }
}

impl MulAssign<f64> for Matrix3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= &rhs;
        self[1] *= &rhs;
        self[2] *= &rhs;
    }
}