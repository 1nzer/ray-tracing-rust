#![allow(dead_code)]

use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(v1: Vec3, v2: Vec3) -> Self {
        Self {
            a: v1,
            b: v2,
        }
    }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
