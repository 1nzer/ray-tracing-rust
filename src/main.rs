#![allow(dead_code)]
#![allow(unused_variables)]
extern crate image;

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod ray;
mod vec3;
mod hittable;
mod sphere;
mod hittable_list;

fn main() {
    let nx = 200 * 5;
    let ny = 100 * 5;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HittableList {
        list_size: 2,
        list: vec![
            Box::new(Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 }),
            Box::new(Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 }),
        ],
    };

    let mut image_buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let u = x as f64 / nx as f64; // = i / nx
        let v = (ny - y - 1) as f64 / ny as f64; // = j / ny
        let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

        let p = r.point_at_parameter(2.0);
        let vec = color(r, &world);

        let ir = (255.99 * vec.r()) as u8;
        let ig = (255.99 * vec.g()) as u8;
        let ib = (255.99 * vec.b()) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();
}

fn color(r: Ray, world: &impl Hittable) -> Vec3 {
    let hit_record_option = world.hit(&r, 0.0, f64::MAX);
    return match hit_record_option {
        Some(hit_record) => {
            Vec3::new(hit_record.normal.x() + 1.0, hit_record.normal.y() + 1.0, hit_record.normal.z() + 1.0) * 0.5
        }
        _ => {
            let v = r.direction();
            let unit_direction = v.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    };
}
