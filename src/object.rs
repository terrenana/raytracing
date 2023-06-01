use std::rc::Rc;

use crate::material::*;
use crate::ray::Ray;
use glam::Vec3;

type Point = Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point,
        material: Rc<dyn Material>,
        outward_normal: Vec3,
        t: f32,
        ray: Ray,
    ) -> Self {
        let mut s = HitRecord {
            point,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material,
            t,
            front_face: false,
        };
        s.set_face_normal(ray, outward_normal);
        s
    }
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct ObjectList {
    pub objects: Vec<Rc<dyn Object>>,
}

unsafe impl Send for ObjectList {}
unsafe impl Sync for ObjectList {}

impl ObjectList {
    pub fn new(objects: Vec<Rc<dyn Object>>) -> Self {
        ObjectList { objects }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, t_min, closest) {
                closest = rec.t;
                hit = Some(rec);
            }
        }
        hit
    }
}

pub struct Sphere {
    center: Point,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, self.material.clone(), normal, t, *ray));
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, self.material.clone(), normal, t, *ray));
            }
        }
        None
    }
}
