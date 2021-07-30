use crate::ray::Ray;
use crate::Point3;

pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = ((self.min[a] - r.origin[a]) / r.direction[a])
                .min((self.max[a] - r.origin[a]) / r.direction[a]);
            let t1 = ((self.min[a] - r.origin[a]) / r.direction[a])
                .max((self.max[a] - r.origin[a]) / r.direction[a]);
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Point3::new(
            (box0.min.x).min(box1.min.x),
            (box0.min.y).min(box1.min.y),
            (box0.min.z).min(box1.min.z),
        );
        let large = Point3::new(
            (box0.max.x).min(box1.max.x),
            (box0.max.y).min(box1.max.y),
            (box0.max.z).min(box1.max.z),
        );

        Self::new(small, large)
    }
}
