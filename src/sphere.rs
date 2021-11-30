use crate::{hittable::Hittable, ray::Ray, vec::Point3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        hit_record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant.is_sign_negative() {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);

        true
    }
}
