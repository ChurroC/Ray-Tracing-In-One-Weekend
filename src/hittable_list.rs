use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

// Took me to long to figure out Vec<Box<dyn Hittable>>
// Vec is a growable array that holds heap-allocated objects so not pointers to values but the actual values on heap
// Then Box takes an object and returns a pointer to it. So it can transform something from the stack or heap into a pointer to the heap
// Then Vec holds those pointers which it knows the size or kinda have the whole value since it's a pointer
// This makes a lot of sense because a Hittable could be a Sphere which is 32 bytes compared to a Plane which could be like 48 bytes
// While with Box each entry is 16 bytes for the pointer location or fixed size pointers
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
 
impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
 
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
 
        temp_rec
    }
}