use std::io::{stderr, Write};
use rand::Rng;

mod vec;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use vec::{Vec3, Point3, Color, unit_vector};
use ray::Ray;
use sphere::Sphere;
use hittable::{HitRecord, Hittable, HittableList};
use camera::Camera;
use material::Material;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;
const INF: f32 = std::f32::INFINITY;
const PI: f32 = std::f32::consts::PI;

pub fn random_f32() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min };
    if x > max { return max };
    x
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: usize) -> Color {
    let mut rec = HitRecord::default();
    // We have exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }

    if world.hit(r, 0.001, INF, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1)
        }

        return Color::new(0.0, 0.0, 0.0)
        // let target = rec.p + rec.normal + random_in_hemisphere(&rec.normal);
        // let diff = target - rec.p;
        // let new_r = Ray::new(&rec.p, &diff);
        // return 0.5 * ray_color(&new_r, world, depth - 1)
    }

    let unit_dir = unit_vector(r.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color total by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as usize,
        (256.0 * clamp(g, 0.0, 0.999)) as usize,
        (256.0 * clamp(b, 0.0, 0.999)) as usize,
    );
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3::new(a as f32 + 0.9 * random_f32(), 0.2, b as f32 + 0.9 * random_f32());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_mat = Material::Lambertian(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat > 0.95 {
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    let sphere_mat = Material::Metal(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    let sphere_mat = Material::Dielectric(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Material::Dielectric(1.5);
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = random_scene();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        20.0, 
        IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32, 
        aperture,
        dist_to_focus,
        look_from, 
        look_at, 
        vup,
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanelines remaining: {}", j);
        let _ = stderr().flush();
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_f32()) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
