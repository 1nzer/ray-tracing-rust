#![allow(dead_code)]

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new_from_f64(t: f64, px: f64, py: f64, pz: f64, nx: f64, ny: f64, nz: f64) -> Self {
        Self {
            t,
            p: Vec3::new(px, py, pz),
            normal: Vec3::new(nx, ny, nz),
        }
    }

    pub fn new_from_vec(t: f64, p: Vec3, normal: Vec3) -> Self {
        Self {
            t,
            p,
            normal,
        }
    }
}
