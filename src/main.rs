use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use ray_tracer::camera::Camera;
use ray_tracer::color::*;
use ray_tracer::hittable::{HitRecord, Hittable};
use ray_tracer::hittable_list::HittableList;
use ray_tracer::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracer::ray::*;
use ray_tracer::sphere::Sphere;
use ray_tracer::vec3::*;
use std::rc::Rc;
use std::time::Instant;

use rand::{thread_rng, Rng};

fn ray_color<T: Hittable>(r: Ray, world: &T, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
        if let Some(scatter_data) = rec.mat_ptr.scatter(&r, &rec) {
            return scatter_data.attenuation
                * ray_color(scatter_data.scattered_ray, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn rand_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_mat = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_f64_range(0.0, 0.5);
                    sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    sphere_mat = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}

fn main() {
    //image dimensions
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 10;
    const MAX_DEPTH: usize = 50;

    //world setup

    let world = random_scene();

    //camera setup

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n 255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = thread_rng();
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise})"), //idk why the colors aren't working
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        //eprintln!("Scanlines remaining: {}", j);
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
        pb.inc(1);
    }
    pb.finish_and_clear();
    //eprintln!("Done.");
}
