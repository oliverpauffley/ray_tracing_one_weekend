use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3(1.0, 1.0, 1.0);

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn mul_scalar(&self, t: f64) -> Vec3 {
        Self {
            0: self.0 * t,
            1: self.1 * t,
            2: self.2 * t,
        }
    }

    pub fn div(&self, t: f64) -> Vec3 {
        self.mul_scalar(1.0 / t)
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            0: self.1 * other.2 - self.2 * other.1,
            1: self.2 * other.0 - self.0 * other.2,
            2: self.0 * other.1 - self.1 * other.0,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self.div(self.length())
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

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
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

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ix = (255.999 * self.0) as i64;
        let iy = (255.999 * self.1) as i64;
        let iz = (255.999 * self.2) as i64;
        write!(f, "{} {} {}\n", ix, iy, iz)
    }
}

#[cfg(test)]
mod test_vec {
    use super::*;

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.mul_scalar(5.0), Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(4.0, 2.0, 6.0);

        assert_eq!(v.div(2.0), Vec3::new(2.0, 1.0, 3.0));
    }
}

// TODO: test dot and cross functions