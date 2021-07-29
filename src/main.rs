use graphics::camera::Camera;
use graphics::hittable_list::HittableList;
use graphics::image::Image;
use graphics::material::{Dielectric, Lambertian};
use graphics::scenes::random_scene;
use graphics::sphere::Sphere;
use graphics::Color;
use graphics::Point3;
use std::sync::Arc;
use vek::vec::Vec3;
fn main() {
    //image dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
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
        0.0..1.0,
    );

    let im = Image::render_scene(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        cam,
        world,
    );
    println!("{}", im);
}
