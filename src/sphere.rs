use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Point3};
 
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let orig_cen = self.center - r.origin();
        let a = r.direction().dot(r.direction());
        let half_b = orig_cen.dot(r.direction());
        let c = orig_cen.dot(orig_cen) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        }

        let sqrt_discriminant = f64::sqrt(discriminant);
        // This is due to +- sqrt(discriminant) and we need to find values in proper bounds
        let mut root = (half_b - sqrt_discriminant) / a;
        if root <= t_min || root >= t_max {
            root = (half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit_record = HitRecord::new(
            t,
            r.at(root),
        );
        hit_record.set_face_normal(r, (r.at(root) - self.center) / self.radius);
        Some(hit_record)
    }
}