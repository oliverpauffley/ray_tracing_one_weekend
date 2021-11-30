use std::{f64::INFINITY, rc::Rc};

use rand::{self, Rng};

use crate::{
    camera::Camera,
    sphere::Sphere,
    vec::{Color, Point3, Vec3},
};
use hittable::{hits, HitRecord, Hittable};
use ray::Ray;

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;

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

    // camera
    let cam = Camera::new();

    // random number generator
    let mut rng = rand::thread_rng();

    // render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Color::ZERO;
            for _ in 0..SAMPLES_PER_PIXEL {
                let randx: f64 = rng.gen();
                let randy: f64 = rng.gen();

                let u: f64 = (i as f64 + randx) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + randy) / (IMAGE_HEIGHT - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_colour = pixel_colour + ray_color(&r, &world);
            }
            pixel_colour.write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}
