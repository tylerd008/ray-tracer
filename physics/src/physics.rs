use crate::Point3;
use vek::vec::Vec3;

pub struct Physics {
    pub mass: f64,
    pub forces: Vec3,
    pub vel: Vec3,
}

pub trait Physical {
    fn get_mass(&self) -> f64;
    fn get_pos(&self, time: f64) -> Point3;
}
