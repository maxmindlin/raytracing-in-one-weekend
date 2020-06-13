use crate::vec::{Point3, Vec3};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self { orig: *orig, dir: *dir }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}