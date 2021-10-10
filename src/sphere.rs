use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Hittable for Sphere<M> {
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

impl<M: Material> Sphere<M> {
    pub fn new(x: f64, y: f64, z: f64, r: f64, m: M) -> Self {
        Self {
            center: Vec3::new(x, y, z),
            radius: r,
            material: m,
        }
    }

    fn get_hit_record(&self, t: f64, r: &Ray) -> Option<HitRecord> {
        let pap = r.point_at_parameter(t);
        let rec = HitRecord {
            t,
            p: pap,
            normal: (pap - self.center) / self.radius,
            material: &self.material,
        };
        return Option::Some(rec);
    }
}
