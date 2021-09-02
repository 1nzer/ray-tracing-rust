#![allow(dead_code)]
#![allow(unused_variables)]
extern crate image;

use crate::vec3::Vec3;

mod ray;
mod vec3;


fn main() {
    let nx = 200;
    let ny = 100;
    let mut image_buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let r = x as f64 / nx as f64;
        let g = (ny - y - 1) as f64 / ny as f64;
        let b = 0.2;

        let v = Vec3::new(r, g, b);

        let ir = (255.99 * v.r()) as u8;
        let ig = (255.99 * v.g()) as u8;
        let ib = (255.99 * v.b()) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();

    let vec1 = Vec3::new(1., 2., 3.);
    let vec2 = Vec3::new(3., 2., 1.);
    let result_vec = vec1 * vec2;
    println!("{}", result_vec)
}

fn color(r: ray::Ray) -> Vec3 {
    Vec3::new(1., 2., 3.)
}
