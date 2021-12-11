use std::ops::{Add, Div, Mul, Sub};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3(1.0, 1.0, 1.0);

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            0: self.1 * other.2 - self.2 * other.1,
            1: self.2 * other.0 - self.0 * other.2,
            2: self.0 * other.1 - self.1 * other.0,
        }
    }
    pub fn unit_vector(&self) -> Self {
        *self / (self.length())
    }

    pub fn new_random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            0: rng.gen_range(min..max),
            1: rng.gen_range(min..max),
            2: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::new_random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut p = Vec3::new_random(-1.0, 1.0);
            p.2 = 0.0;
            if p.length_squared() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    /// near_zero returns true if the vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal * (self.dot(normal) * 2.0)
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let negative_vec = *self * -1.0;
        let cos_theta = negative_vec.dot(n).min(1.0);
        let r_out_perp = (*self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).sqrt();
        r_out_perp + r_out_parallel
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

#[cfg(test)]
mod test_vec {
    use super::*;

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v * (5.0), Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(4.0, 2.0, 6.0);

        assert_eq!(v / (2.0), Vec3::new(2.0, 1.0, 3.0));
    }
}

// TODO: test dot and cross functions
