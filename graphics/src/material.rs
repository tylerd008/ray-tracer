use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::*;
use crate::Color;
use rand::{thread_rng, Rng};
use vek::vec::Vec3;

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        let mut scatter_direction = rec.normal + random_unit_vector(); //feel like i should be able to write this without needing it to be mut

        if scatter_direction.is_approx_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterData {
            scattered_ray: Ray::new(rec.p, scatter_direction, r_in.time),
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
        let reflected = unit_vector(r_in.direction).reflected(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );

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

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let mut rng = thread_rng();

        let unit_direction = unit_vector(r_in.direction);

        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).powf(0.5);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        //this is true, then the snell's law equation has no solutions and so the ray reflects instead of refracting
        let direction = if cannot_refract
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0))
        {
            unit_direction.reflected(rec.normal)
        } else {
            unit_direction.refracted(rec.normal, refraction_ratio)
        };

        Some(ScatterData {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered_ray: Ray::new(rec.p, direction, r_in.time),
        })
    }
}
