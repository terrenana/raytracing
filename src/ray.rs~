use crate::vector::Vec3;

pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.o + self.d * t
    }
}
