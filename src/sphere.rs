use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
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
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

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
