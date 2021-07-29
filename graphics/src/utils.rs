use rand::{thread_rng, Rng};
use vek::vec::Vec3;

pub fn unit_vector(vec: Vec3<f64>) -> Vec3<f64> {
    vec / vec.magnitude()
}

pub fn random_vec_range(min: f64, max: f64) -> Vec3<f64> {
    let mut rng = thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_vec() -> Vec3<f64> {
    random_vec_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3<f64> {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.magnitude_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3<f64> {
    let mut rng = thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.magnitude_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3<f64> {
    unit_vector(random_in_unit_sphere())
}
pub fn rand_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn rand_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}
