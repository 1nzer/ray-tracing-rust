#![allow(dead_code)]
#![allow(unused_variables)]

mod vec3;
mod ray;

extern crate image;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut image_buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let r = x as f64 / nx as f64;
        let g = (ny - y - 1) as f64 / ny as f64;
        let b = 0.2;
        let ir = (255.99 * r) as u8;
        let ig = (255.99 * g) as u8;
        let ib = (255.99 * b) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    image_buf.save("./tmp/image.png").unwrap();

    let vec1 = vec3::Vec3::new(1.,2.,3.);
    let vec2 = vec3::Vec3::new(3.,2., 1.);
    let result_vec = vec1 * vec2;
    println!("{}", result_vec)
}

fn color(r: ray::Ray) -> vec3::Vec3 {
    vec3::Vec3::new(1.,2.,3.)
}
