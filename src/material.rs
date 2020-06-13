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

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
    fn clone(&self) -> Box<dyn Material>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_dir = rec.normal + random_unit_vector();
        *scattered = Ray::new(&rec.p, &scatter_dir);
        *attenuation = self.albedo;
        true
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    roughness: f32,
}

impl Metal {
    pub fn new(albedo: Color, r: f32) -> Self {
        Self { albedo, roughness: f32::min(r, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let u = unit_vector(r_in.dir);
        let reflected = reflect(&u, &rec.normal) + self.roughness * random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        dot(&scattered.dir, &rec.normal) > 0.0
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.albedo, self.roughness))
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let etai_over_etat = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };

        let unit_dir = unit_vector(r_in.dir);

        let dotted = dot(&-unit_dir, &rec.normal);
        let cos_theta = f32::min(dotted, 1.0);
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
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.ref_idx))
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
