use graphics::camera::Camera;
use graphics::ray::Point3;
use graphics::render_scene;
use graphics::scenes::random_scene;
use vek::vec::Vec3;

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

    render_scene(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        cam,
        world,
    )
}
