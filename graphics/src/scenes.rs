use crate::camera::Camera;
use crate::color::*;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::*;
use crate::render_scene;
use crate::sphere::Sphere;
use crate::utils::*;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::time::Instant;

use rand::{thread_rng, Rng};

fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn rand_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
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

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let sphere_mat: Arc<dyn Material + Send + Sync>;
                if choose_mat < 0.8 {
                    let albedo = random_vec() * random_vec();
                    sphere_mat = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = rand_f64_range(0.0, 0.5);
                    sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    sphere_mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
