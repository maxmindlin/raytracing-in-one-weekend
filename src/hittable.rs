use crate::ray::Ray;
use crate::vec::{Vec3, Point3, dot};
use crate::material::Material;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Box<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

// Used to determine if a given
// object is "hittable" and therefore
// if it is hit by a given ray.
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if let Some(temp_res) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_res.t;
                hit_anything = Some(temp_res);
            }
        }

        hit_anything
    }
}