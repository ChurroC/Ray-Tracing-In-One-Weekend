use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
#[derive(Clone)]
pub struct HitRecord {
    intersect_p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool
}
 
impl HitRecord {
    pub fn new(t: f64, intersect_p: Point3) -> HitRecord {
        HitRecord { t, intersect_p, normal: Vec3::default(), front_face: false }
    }
    
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // If the direction and normal are opposing each other meaning front face then dot product is negative
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        // Makes sure the normal always opposes or points toward the viewer
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// Reason to have a rec as a parameter is to avoid reallocation since we can reuse the same one
// instead of making a new one to return. Also we only modify on a hit.
// But on this I switched to returning a optional HitRecord since this is a fixed struct then it will be on the stack.
// Also learned that stack is faster since we adjust a pointer vs searching for memory on the heap.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}