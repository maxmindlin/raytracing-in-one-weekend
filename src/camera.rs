use crate::vec::{Vec3, Point3, unit_vector, cross, random_in_unit_disk};
use crate::ray::Ray;
use crate::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        vfov: f32, 
        aspect_ratio: f32, 
        aperture: f32,
        focus_dist: f32,
        look_from: Point3, 
        look_at: Point3, 
        vup: Vec3
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            v,
            u,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // let v = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        // Ray::new(&self.origin, &v)
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        
        let o = self.origin + offset;
        let v = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        Ray::new(&o, &v)
    }
}