use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
#[derive(Clone, Default)]
pub struct HitRecord {
    intersect_p: Point3,
    normal: Vec3,
    t: f64,
}
 
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
}
 
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}