use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        // catch errors where the scatter is the normal
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let origin = hit_record.p;
        let direction = scatter_direction;
        let r_out = Ray::new(origin, direction);
        Some((self.albedo, r_out))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::unit_vector(&r_in.direction).reflect(hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
