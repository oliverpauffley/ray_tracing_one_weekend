use std::{panic, rc::Rc};

use crate::{
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl std::fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HitRecord")
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("t", &self.t)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl HitRecord {
    pub fn new(
        r: Ray,
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let (front_face, normal) = set_face_normal(r, outward_normal);
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}
pub fn set_face_normal(r: Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = r.direction.dot(outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        outward_normal * -1.0
    };
    (front_face, normal)
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub fn hits(
    hittables: &Vec<Rc<dyn Hittable>>,
    r: Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    for object in hittables.iter() {
        if let Some(record) = object.hit(r, t_min, t_max) {
            return Some(record);
        }
    }
    None
}
