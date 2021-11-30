use std::{f64::INFINITY, rc::Rc};

use crate::{
    sphere::Sphere,
    vec::{Color, Point3, Vec3},
};
use hittable::{hits, HitRecord, Hittable};
use ray::Ray;

mod color;
mod hittable;
mod ray;
mod sphere;
mod vec;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn ray_color(r: &Ray, world: &Vec<Rc<dyn Hittable>>) -> Color {
    let mut record = HitRecord::new();
    if hits(&world, r, 0.0, INFINITY, &mut record) {
        return (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    // produce a blended blue background in the y direction.
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // world
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    world.push(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.push(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner =
        origin - horizontal / (2.0) - vertical / (2.0) - Vec3::new(0.0, 0.0, focal_length);

    // render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let c = ray_color(&r, &world);
            c.write_color();
        }
    }

    eprintln!("Done");
}
