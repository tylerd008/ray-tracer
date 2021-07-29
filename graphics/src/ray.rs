use crate::Point3;
use vek::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3<f64>, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
