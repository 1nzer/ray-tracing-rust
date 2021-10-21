use rand::Rng;

use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Sync {
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
        return (self.albedo, scattered, scattered.direction().dot(rec.normal) > 0.0);
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self {
            ref_idx: ri,
        }
    }

    pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> (bool, Vec3) {
        let uv = v.unit_vector();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
            return (true, refracted);
        }
        return (false, Vec3::new(0.0, 0.0, 0.0));
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * v.dot(n) * 1.0;
    }

    pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let outward_normal;
        let reflected = Self::reflect(r_in.direction(), rec.normal);
        let ni_over_nt;
        let cosine;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        if r_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = rec.normal * -1.0;
            ni_over_nt = self.ref_idx;
            cosine = (r_in.direction().dot(rec.normal) * self.ref_idx) / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = (r_in.direction().dot(rec.normal) * -1.0) / r_in.direction().length();
        }
        let (is_refracted, refracted) = Self::refract(r_in.direction(), outward_normal, ni_over_nt);
        let reflect_prod;
        if is_refracted {
            reflect_prod = Self::schlick(cosine, self.ref_idx);
        } else {
            reflect_prod = 1.0;
        }
        let mut rng = rand::thread_rng();
        let scattered;
        if rng.gen::<f64>() < reflect_prod {
            scattered = Ray::new(rec.p, reflected);
        } else {
            scattered = Ray::new(rec.p, refracted);
        }

        return (attenuation, scattered, true);
    }
}
