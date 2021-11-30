use std::rc::Rc;

use crate::{
    ray::Ray,
    vec::{Point3, Vec3},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub fn hits(
    hittables: &Vec<Rc<dyn Hittable>>,
    r: &Ray,
    t_min: f64,
    t_max: f64,
    hit_record: &mut HitRecord,
) -> bool {
    let mut hit_anything = false;

    hittables.iter().for_each(|object| {
        let co = object.clone();
        if co.hit(r, t_min, t_max, hit_record) {
            hit_anything = true;
        }
    });

    hit_anything
}
