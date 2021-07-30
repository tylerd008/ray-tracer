use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
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
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
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
    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;
        let mut temp_box: Option<AABB>;

        let mut first_box = true;
        for object in &self.objects {
            temp_box = object.bounding_box(time.clone());
            if let None = temp_box {
                return None;
            }

            output_box = if first_box {
                temp_box
            } else {
                Some(AABB::surrounding_box(
                    output_box.unwrap(),
                    temp_box.unwrap(),
                ))
            };
            first_box = false;
        }

        output_box
    }
}
