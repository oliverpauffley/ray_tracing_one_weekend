use crate::vec::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction.mul_scalar(t)
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_at_origin() {
        // test that the ray at the origin with ray parameter zero returns the origin
        let origin = Point3::ZERO;
        let direction = Vec3::ONE;
        let ray = Ray::new(origin, direction);

        let t1 = 0.0;
        assert_eq!(ray.at(t1), origin);

        let t2 = 1.0;
        assert_eq!(ray.at(t2), Point3::ONE);

        let t3 = 2.0;
        assert_eq!(ray.at(t3), Point3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(origin, direction);

        let t1 = 1.0;
        assert_eq!(ray.at(t1), Point3::new(2.0, 4.0, 6.0));

        let t2 = 2.0;
        assert_eq!(ray.at(t2), Point3::new(3.0, 6.0, 9.0));
    }
}
