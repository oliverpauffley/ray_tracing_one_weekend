use std::panic;

use rand::Rng;

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
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::unit_vector(&r_in.direction).reflect(hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );

        if scattered.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation of reflectance
        let r = ((1.0 - ref_idx) / (1.0 + ref_idx)).powf(2.0);
        r + (1.0 - r) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();

        let attenuation = Color::ONE;
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(hit_record.normal)
            } else {
                unit_direction.refract(hit_record.normal, refraction_ratio)
            };

        let scattered = Ray::new(hit_record.p, direction);
        Some((attenuation, scattered))
    }
}
