use std::{f64::INFINITY, rc::Rc};

use material::Material;
use rand::{self, Rng};

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec::{Color, Point3, Vec3},
};
use hittable::{hits, Hittable};
use ray::Ray;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: usize = 500;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: Ray, world: &[Rc<dyn Hittable>], depth: i32) -> Color {
    // we have reached the max ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::ZERO;
    }

    // if we get a hit
    if let Some(record) = hits(world, r, 0.001, INFINITY) {
        // and it hits some material
        if let Some((attenuation, scattered)) = record.material.scatter(r, &record) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::ZERO;
    }
    // we didnt hit an object so we produce a blended blue background in the y direction.
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // world
    let world = random_scene();

    // camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::ZERO;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
                pixel_colour = pixel_colour + ray_color(r, &world, MAX_DEPTH);
            }
            pixel_colour.write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}

fn random_scene() -> Vec<Rc<dyn Hittable>> {
    let mut rng = rand::thread_rng();

    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();

    for a in -11..=10 {
        for b in -11..=10 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let a_flt = a as f64;
            let b_flt = b as f64;
            let center = Point3::new(
                a_flt + 0.9 * rng.gen::<f64>(),
                0.2,
                b_flt + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat = random_material(choose_mat);
                world.push(Rc::new(Sphere::new(center, 0.2, mat)));
            };
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.push(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));
    world.push(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));
    world.push(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    world
}

fn random_material(random_number: f64) -> Rc<dyn Material> {
    let mut rng = rand::thread_rng();
    match random_number {
        x if (0.0..0.8).contains(&x) => {
            // diffuse
            let albedo = Color::new_random(0.0, 1.0) * Color::new_random(0.0, 1.0);
            Rc::new(Lambertian::new(albedo))
        }
        x if (0.8..0.9).contains(&x) => {
            // metal
            let albedo = Color::new_random(0.5, 1.0);
            let fuzz = rng.gen_range(0.0..=0.5);
            Rc::new(Metal::new(albedo, fuzz))
        }
        _ => {
            // glass
            Rc::new(Dielectric::new(1.5))
        }
    }
}
