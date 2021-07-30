use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::{Movement, Sphere};
use crate::utils::*;
use crate::Color;
use crate::Point3;
use std::ops::Range;
use std::sync::Arc;

pub fn random_scene(time: &Option<Range<f64>>) -> HittableList {
    let mut world = HittableList::new();
    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
        None,
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
                    let end_point = center + Point3::new(0.0, rand_f64_range(0.0, 0.5), 0.0);
                    let mvmt = if let Some(t) = time {
                        Some(Movement::new(t.clone(), end_point))
                    } else {
                        None
                    };
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat, mvmt)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = rand_f64_range(0.0, 0.5);
                    sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat, None)));
                } else {
                    sphere_mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat, None)));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
        None,
    )));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
        None,
    )));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
        None,
    )));

    world
}
