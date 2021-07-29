use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::Point3;
use std::sync::Arc;
use vek::vec::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.magnitude_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.powf(0.5);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord {
            t: root,
            p: p,
            normal: HitRecord::set_face_normal(r, outward_normal),
            mat_ptr: self.mat_ptr.clone(),
            front_face: Vec3::dot(r.direction, outward_normal) < 0.0,
        })
    }
}
