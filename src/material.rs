use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterData>;
}

pub struct Lambertian {
    albedo: Color,
}

pub struct ScatterData {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(); //feel like i should be able to write this without needing it to be mut

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterData {
            scattered_ray: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some(ScatterData {
                scattered_ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
