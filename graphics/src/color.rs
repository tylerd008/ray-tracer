use vek::vec::Vec3;

pub type Color = Vec3<f64>;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).powf(0.5);
    g = (scale * g).powf(0.5);
    b = (scale * b).powf(0.5);

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize
    )
}
