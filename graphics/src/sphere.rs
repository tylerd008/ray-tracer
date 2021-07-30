use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::Point3;
use std::ops::Range;
use std::sync::Arc;
use vek::vec::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material + Send + Sync>,
    movement: Option<Movement>,
}

pub struct Movement {
    time: Range<f64>,
    end_point: Point3,
}

impl Movement {
    pub fn new(time: Range<f64>, end_point: Point3) -> Self {
        Self { time, end_point }
    }
}

impl Sphere {
    pub fn new(
        center: Point3,
        radius: f64,
        mat_ptr: Arc<dyn Material + Send + Sync>,
        movement: Option<Movement>,
    ) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
            movement,
        }
    }
}

impl Sphere {
    pub fn center(&self, t: f64) -> Point3 {
        if let Some(mvmt) = &self.movement {
            self.center
                + ((t - mvmt.time.start) / (mvmt.time.end - mvmt.time.start))
                    * (mvmt.end_point - self.center)
        } else {
            self.center
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = if let Some(_) = self.movement {
            r.origin - self.center(r.time)
        } else {
            r.origin - self.center
        };
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
        let outward_normal = if let Some(_) = self.movement {
            //maybe roll the 2 if lets into 1 somehow
            p - self.center(r.time) / self.radius
        } else {
            (p - self.center) / self.radius
        };

        Some(HitRecord {
            t: root,
            p: p,
            normal: HitRecord::set_face_normal(r, outward_normal),
            mat_ptr: self.mat_ptr.clone(),
            front_face: Vec3::dot(r.direction, outward_normal) < 0.0,
        })
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        let output = if let Some(_) = self.movement {
            let box1 = AABB::new(
                self.center(time.start) - Point3::new(self.radius, self.radius, self.radius),
                self.center(time.start) + Point3::new(self.radius, self.radius, self.radius),
            );
            let box2 = AABB::new(
                self.center(time.end) - Point3::new(self.radius, self.radius, self.radius),
                self.center(time.end) + Point3::new(self.radius, self.radius, self.radius),
            );
            AABB::surrounding_box(box1, box2)
        } else {
            AABB::new(
                self.center - Point3::new(self.radius, self.radius, self.radius),
                self.center + Point3::new(self.radius, self.radius, self.radius),
            )
        };
        Some(output)
    }
}
