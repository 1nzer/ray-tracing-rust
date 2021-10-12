use std::f64::consts::PI;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, v_fow: f64, aspect: f64) -> Self {
        let theta = v_fow * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);
        Self {
            lower_left_corner: origin - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
            origin,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin)
    }
}
