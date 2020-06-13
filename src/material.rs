use crate::random_f32;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec::{
    Color, 
    random_unit_vector, 
    reflect, 
    refract,
    unit_vector, 
    dot, 
    random_in_unit_sphere,
};

#[derive(Clone, Copy)]
pub enum Material {
    Dielectric(f32),
    Lambertian(Color),
    Metal(Color, f32),
    Empty,
}

impl Default for Material {
    fn default() -> Self {
        Self::Empty
    }
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Self::Lambertian(c) => {
                let scatter_dir = rec.normal + random_unit_vector();
                *scattered = Ray::new(&rec.p, &scatter_dir);
                *attenuation = c.clone();
                true
            },
            Self::Metal(c, f) => {
                let u = unit_vector(r_in.dir);
                let reflected = reflect(&u, &rec.normal) + *f * random_in_unit_sphere();
                *scattered = Ray::new(&rec.p, &reflected);
                *attenuation = c.clone();
                dot(&scattered.dir, &rec.normal) > 0.0
            },
            Self::Dielectric(idx) => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let etai_over_etat = match rec.front_face {
                    true => 1.0 / idx,
                    false => *idx,
                };

                let unit_dir = unit_vector(r_in.dir);

                let dotted = dot(&-unit_dir, &rec.normal);

                // f32 doesnt implement Ord, so you cant use cmp::min ? >:(
                let cos_theta = if dotted > 1.0 { 1.0 } else { dotted };
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
                if (etai_over_etat * sin_theta) > 1.0 {
                    let reflected = reflect(&unit_dir, &rec.normal);
                    *scattered = Ray::new(&rec.p, &reflected);
                    return true
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if random_f32() < reflect_prob {
                    let reflected = reflect(&unit_dir, &rec.normal);
                    *scattered = Ray::new(&rec.p, &reflected);
                    return true
                }

                let refracted = refract(&unit_dir, &rec.normal, etai_over_etat);
                *scattered = Ray::new(&rec.p, &refracted);

                true
            },
            Self::Empty => false
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
