use ray_tracer::camera::Camera;
use ray_tracer::color::*;
use ray_tracer::hittable::{HitRecord, Hittable};
use ray_tracer::hittable_list::HittableList;
use ray_tracer::ray::*;
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::*;
use std::rc::Rc;

use rand::{thread_rng, Rng};

fn ray_color<T: Hittable>(r: Ray, world: &T, depth: usize) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Point3::random_unit_vector();
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    //image dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    //world setup
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //camera setup
    let cam = Camera::new();

    println!("P3\n{} {}\n 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let rand1: f64 = rng.gen_range(0.0..1.0);
                let rand2: f64 = rng.gen_range(0.0..1.0);
                let u: f64 = (i as f64 + rand1) / ((IMAGE_WIDTH - 1) as f64);
                let v: f64 = (j as f64 + rand2) / ((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}
