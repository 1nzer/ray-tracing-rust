use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0 as f64 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pap = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pap,
                    normal: (pap - self.center) / self.radius
                };
                return Option::Some(rec);
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let pap = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pap,
                    normal: (pap - self.center) / self.radius
                };
                return Option::Some(rec);
            }
        }
        return Option::None
    }
}
