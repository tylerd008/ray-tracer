use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.e0;
    let mut g = pixel_color.e1;
    let mut b = pixel_color.e2;

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as usize,
        (256.0 * clamp(g, 0.0, 0.999)) as usize,
        (256.0 * clamp(b, 0.0, 0.999)) as usize
    )
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
