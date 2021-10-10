use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Vec3, Ray, bool); // attenuation, scattered, scatter
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        return (self.albedo, scattered, true);
    }
}

impl Lambertian {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            albedo: Vec3::new(x, y, z),
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(x: f64, y: f64, z: f64, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Self {
            albedo: Vec3::new(x, y, z),
            fuzz,
        }
    }
    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * v.dot(n) * 2.0;
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = Self::reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        return (self.albedo, scattered, scattered.direction().dot(rec.normal) > 0.0)
    }
}
