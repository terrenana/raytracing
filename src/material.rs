use crate::object::*;
use crate::ray::*;
use crate::util::*;
use glam::Vec3;
use rand::random;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = hit.normal + random_unit_vector();
        if near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }
        Some((Ray::new(hit.point, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected: Vec3 = reflect(r_in.direction.normalize(), hit.normal);
        let scattered = Ray::new(hit.point, reflected + self.fuzz * random_in_unit_sphere());

        if scattered.direction.dot(hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f32,
}
impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Dielectric { ir }
    }
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = match hit.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_dir = r_in.direction.normalize();

        let cos_theta = -unit_dir.dot(hit.normal).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta.powi(2));

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random::<f32>() {
            direction = reflect(unit_dir, hit.normal);
        } else {
            direction = refract(unit_dir, hit.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit.point, direction);
        Some((scattered, Vec3::new(1.0, 1.0, 1.0)))
    }
}
