use std::f64::consts::PI;

use rand::Rng;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, v_fow: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = v_fow * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);
        Self {
            lower_left_corner: origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            horizontal: u * half_width * 2.0 * focus_dist,
            vertical: v * half_height * 2.0 * focus_dist,
            origin,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Self::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset
        )
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p;
        loop {
            p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }
}
