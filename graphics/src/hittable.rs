use crate::material::Material;
use crate::ray::{Point3, Ray};
use std::rc::Rc;
use std::sync::Arc;
use vek::vec::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3<f64>,
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(r: &Ray, outward_normal: Vec3<f64>) -> Vec3<f64> {
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
