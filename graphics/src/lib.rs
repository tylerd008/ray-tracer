pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod scenes;
pub mod sphere;
mod utils;

use camera::Camera;
use color::write_color;
use color::Color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use utils::*;

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use vek::vec::Vec3;

pub fn render_scene(
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    cam: Camera,
    world: HittableList,
) {
    println!("P3\n{} {}\n 255", image_width, image_height);

    let mut rng = thread_rng();
    let pb = ProgressBar::new(image_width as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise})"), //idk why the colors aren't working
    );

    for j in (0..image_height).rev() {
        //eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let rand1: f64 = rng.gen_range(0.0..1.0);
                let rand2: f64 = rng.gen_range(0.0..1.0);
                let u: f64 = (i as f64 + rand1) / ((image_width - 1) as f64);
                let v: f64 = (j as f64 + rand2) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
}

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
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
