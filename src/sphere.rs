use crate::vec::{Point3, dot};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub mat: Material,
}

impl Sphere{
    pub fn new(center: Point3, radius: f32, mat: Material) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    // Define how to determine if a ray
    // has hit a sphere
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_sqrd();
        let half_b = dot(&oc, &r.dir);
        let c = oc.length_sqrd() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.mat;
                return true
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat = self.mat;
                return true
            }
        }

        false
    }
}