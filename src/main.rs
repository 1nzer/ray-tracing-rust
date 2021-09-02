#![allow(dead_code)]
#![allow(unused_variables)]
extern crate image;

use crate::ray::Ray;
use crate::vec3::Vec3;

mod ray;
mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut image_buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let u = x as f64 / nx as f64; // = i / nx
        let v = (ny - y - 1) as f64 / ny as f64; // = j / ny

        let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

        let vec = color(r);

        let ir = (255.99 * vec.r()) as u8;
        let ig = (255.99 * vec.g()) as u8;
        let ib = (255.99 * vec.b()) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0)
    }
    let v = r.direction();
    let unit_direction = v.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
