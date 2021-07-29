use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::utils::*;
use crate::Color;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

use std::fmt;

use indicatif::{FormattedDuration, ProgressBar, ProgressStyle};

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Pixel>,
    image_height: usize,
    image_width: usize,
}

impl Image {
    pub fn render_scene(
        image_width: usize,
        image_height: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        cam: Camera,
        world: HittableList,
    ) -> Self {
        const THREAD_NUM: usize = 4;
        let image_data: Vec<Vec<Pixel>> = vec![Vec::new(); 4];
        let id = Arc::new(Mutex::new(image_data));

        let pb = ProgressBar::new(image_height as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise})"), //idk why the colors aren't working
        );

        let beginning = Instant::now();
        let mut handles = vec![];
        let mut start = 0;
        for z in 0..THREAD_NUM {
            let cam = cam.clone();
            let world = world.clone();
            let id = Arc::clone(&id);
            let pb = pb.clone();
            let end =
                (image_height as f64 * (((z + 1) as f64) / THREAD_NUM as f64)).ceil() as usize;
            let handle = thread::spawn(move || {
                eprintln!("for z: {}, start: {}, end: {} ", z, start, end);
                for j in (start..end).rev() {
                    for i in 0..image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let rand1: f64 = rand_f64();
                            let rand2: f64 = rand_f64();
                            let u: f64 = (i as f64 + rand1) / ((image_width - 1) as f64);
                            let v: f64 = (j as f64 + rand2) / ((image_height - 1) as f64);
                            let r = cam.get_ray(u, v);
                            pixel_color += ray_color(r, &world, max_depth);
                        }
                        let pixel = Pixel::write_color(pixel_color, samples_per_pixel);
                        {
                            let mut image_data = id.lock().unwrap();
                            image_data[z].push(pixel);
                        }
                    }
                    pb.inc(1);
                }
            });
            start = end;
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let mut output = Self {
            pixels: Vec::new(),
            image_height,
            image_width,
        };
        let image_data = id.lock().unwrap();
        for i in (0..THREAD_NUM).rev() {
            for pixl in &image_data[i] {
                output.pixels.push(*pixl);
            }
        }
        pb.finish_and_clear();
        eprintln!(
            "Scene rendered in {}",
            FormattedDuration(beginning.elapsed())
        );
        output
    }
}

impl Pixel {
    pub fn write_color(pixel_color: Color, samples_per_pixel: usize) -> Self {
        let mut r = pixel_color.x;
        let mut g = pixel_color.y;
        let mut b = pixel_color.z;

        let scale = 1.0 / (samples_per_pixel as f64);
        r = (scale * r).powf(0.5);
        g = (scale * g).powf(0.5);
        b = (scale * b).powf(0.5);

        Self {
            r: (256.0 * r.clamp(0.0, 0.999)) as u8,
            g: (256.0 * g.clamp(0.0, 0.999)) as u8,
            b: (256.0 * b.clamp(0.0, 0.999)) as u8,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!(
            "P3\n{} {}\n 255\n",
            self.image_width, self.image_height
        ));
        for pixel in &self.pixels {
            output.push_str(&format!("{}\n", pixel));
        }
        write!(f, "{}", output)
    }
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
