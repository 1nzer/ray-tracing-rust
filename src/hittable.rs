#![allow(dead_code)]

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
