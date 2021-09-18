#![allow(dead_code)]
#![allow(unused_variables)]
extern crate image;

use rand::Rng;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut rng = rand::thread_rng();

    let world = HittableList::new(vec![
        Box::new(Sphere::new(0.0, 0.0, -1.0, 0.5)),
        Box::new(Sphere::new(0.0, -100.5, -1.0, 100.0)),
    ]);

    let cam = Camera::new();

    let mut image_buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let i = x as f64;
        let j = (ny - y - 1) as f64;

        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for s in 0..ns {
            let u = (i + rng.gen::<f64>()) / nx as f64;
            let v = (j + rng.gen::<f64>()) / ny as f64;

            let r = cam.get_ray(u, v);
            let p = r.point_at_parameter(2.0);
            col = color(r, &world) + col;
        }

        col = col / ns as f64;
        let ir = (255.99 * col.e[0].sqrt()) as u8;
        let ig = (255.99 * col.e[1].sqrt()) as u8;
        let ib = (255.99 * col.e[2].sqrt()) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();
}

fn color<T: Hittable>(r: Ray, world: &T) -> Vec3 {
    let hit_record_option = world.hit(&r, 0.001, f64::MAX);
    return match hit_record_option {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            color(Ray::new(rec.p, target - rec.p), world) * 0.5
        }
        _ => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    };
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}
