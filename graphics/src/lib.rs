pub mod aabb;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod material;
pub mod ray;
pub mod scenes;
pub mod sphere;
mod utils;

use vek::vec::Vec3;

pub type Color = Vec3<f64>;
pub type Point3 = Vec3<f64>;
