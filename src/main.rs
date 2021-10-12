#![allow(dead_code)]
#![allow(unused_variables)]
extern crate image;

use rand::Rng;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod material;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut rng = rand::thread_rng();

    let world = random_scene();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

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
            col = color(r, &world, 0) + col;
        }

        col = col / ns as f64;
        let ir = (255.99 * col.e[0].sqrt()) as u8;
        let ig = (255.99 * col.e[1].sqrt()) as u8;
        let ib = (255.99 * col.e[2].sqrt()) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();
}

fn color<T: Hittable>(r: Ray, world: &T, depth: i32) -> Vec3 {
    let hit_record_option = world.hit(&r, 0.001, f64::MAX);
    return match hit_record_option {
        Some(rec) => {
            let (attenuation, scattered, scatter) = rec.material.scatter(r, &rec);
            if depth < 50 && scatter {
                return attenuation * color(scattered, world, depth + 1);
            }
            return Vec3::new(0.0, 0.0, 0.0);
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

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let n = 500;
    let mut list = HittableList { list: vec![] };
    list.list.push(Box::new(Sphere::new(0.0, -1000.0, 0.0, 1000.0, Lambertian::new(0.5, 0.5, 0.5))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    list.list.push(
                        Box::new(
                            Sphere::new(
                                center.x(),
                                center.y(),
                                center.z(),
                                0.2,
                                Lambertian::new(
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                ),
                            )
                        )
                    );
                } else if choose_mat < 0.95 { // metal
                    list.list.push(
                        Box::new(
                            Sphere::new(
                                center.x(),
                                center.y(),
                                center.z(),
                                0.2,
                                Metal::new(
                                    0.5 * (1.0 + rng.gen::<f64>()),
                                    0.5 * (1.0 + rng.gen::<f64>()),
                                    0.5 * (1.0 + rng.gen::<f64>()),
                                    0.5 * rng.gen::<f64>(),
                                ),
                            )
                        )
                    );
                } else { // glass
                    list.list.push(
                        Box::new(
                            Sphere::new(
                                center.x(),
                                center.y(),
                                center.z(),
                                0.2,
                                Dielectric::new(1.5),
                            )
                        )
                    );
                }
            }
        }
    }

    list.list.push(Box::new(Sphere::new(0.0, 1.0, 0.0, 1.0, Dielectric::new(1.5))));
    list.list.push(Box::new(Sphere::new(-4.0, 1.0, 0.0, 1.0, Lambertian::new(0.4, 0.2, 0.1))));
    list.list.push(Box::new(Sphere::new(4.0, 1.0, 0.0, 1.0, Metal::new(0.7, 0.6, 0.5, 0.0))));

    return list;
}
