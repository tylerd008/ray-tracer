use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            rec = if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
                Some(record)
            } else {
                continue;
            };
        }
        rec
    }
}
