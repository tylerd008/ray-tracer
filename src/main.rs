use graphics::camera::Camera;
use graphics::color::Color;
use graphics::hittable_list::HittableList;
use graphics::image::Image;
use graphics::material::{Dielectric, Lambertian};
use graphics::ray::Point3;
use graphics::render_scene;
use graphics::scenes::random_scene;
use graphics::sphere::Sphere;
use std::sync::Arc;
use vek::vec::Vec3;
fn main() {
    //image dimensions
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 50;

    //world setup

    let mut world = HittableList::new();

    let mat = Arc::new(Dielectric::new(1.33));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.5, mat)));
    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    //camera setup

    let look_from = Point3::new(3.0, 2.0, 3.0);
    let look_at = Point3::new(3.0, 2.0, -3.0);
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
