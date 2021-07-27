//use crate::vec3::Vec3;
use vek::vec::Vec3;
pub type Point3 = Vec3<f64>;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
