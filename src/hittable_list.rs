use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = Option::None;
        for k in self.list.iter() {
            let temp_rec = k.hit(r, t_min, closest_so_far);
            match temp_rec {
                Some(hit) => {
                    closest_so_far = hit.t;
                    rec = Option::Some(hit);
                }
                _ => {}
            }
        }
        return rec;
    }
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            list,
        }
    }
}
