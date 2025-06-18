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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let orig_cen = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let half_b = orig_cen.dot(r.direction());
        let c = orig_cen.dot(orig_cen) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        // This is due to +- sqrt(discriminant) and we only want the smallest positive root
        let mut root = (-half_b - f64::sqrt(discriminant)) / a;
        if root < t_min || root > t_max {
            root = (-half_b + (-half_b - f64::sqrt(discriminant)) / a) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.intersect_p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius; // Normalized normal vector
        true
    }
}