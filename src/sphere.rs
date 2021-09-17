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
                return self.get_hit_record(temp, r);
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return self.get_hit_record(temp, r);
            }
        }
        return Option::None
    }
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64) -> Self {
        Self {
            center: Vec3::new(x, y, z),
            radius: r,
        }
    }

    fn get_hit_record(&self, t: f64, r: &Ray) -> Option<HitRecord> {
        let pap = r.point_at_parameter(t);
        let rec = HitRecord::new_from_vec(t, pap, (pap - self.center) / self.radius);
        return Option::Some(rec);
    }
}
