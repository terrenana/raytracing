use std::rc::Rc;

use crate::ray::Ray;
use glam::Vec3;

type Point = Vec3;

#[derive(Clone, Copy)]
struct HitRecord {
    point: Point,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
    fn set_face_normal(mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

trait Object {
    fn hit(self: Rc<Self>, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

struct ObjectList {
    objects: Vec<Rc<dyn Object>>,
}

impl ObjectList {
    fn new(object: Rc<dyn Object>) -> Self {
        ObjectList {
            objects: vec![object],
        }
    }
    fn clear(&mut self) {
        self.objects.clear();
    }
    fn add(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for ObjectList {
    fn hit(self: Rc<Self>, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();

        let mut hit_anything = false;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if object.clone().hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *record = temp_record;
            }
        }
        hit_anything
    }
}

struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    fn new(center: Point, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Object for Sphere {
    fn hit(self: Rc<Self>, ray: Ray, t_min: f32, t_max: f32, mut record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_disc = discriminant.sqrt();
        let mut root = (-half_b - sqrt_disc) / a;
        if root < t_min || t_max > root {
            root = (-half_b + sqrt_disc) / a;
            if root < t_min || t_max > root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        true
    }
}
