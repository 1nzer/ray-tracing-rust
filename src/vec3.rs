#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&self) -> Self {
        let k = 1.0 / self.length();
        Self::new(self.e[0] * k, self.e[1] * k, self.e[2] * k)
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3::new(
            v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
            -(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0]),
            v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs)
    }
}


impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.e[0] - rhs, self.e[1] - rhs, self.e[2] - rhs)
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}


impl Display for Vec3 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        println!("{} {} {}", self.e[0], self.e[1], self.e[2]);
        Result::Ok(())
    }
}


