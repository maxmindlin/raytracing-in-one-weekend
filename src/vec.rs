use std::ops;
use rand::Rng;

use crate::{PI, random_f32};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
        }
    }

    pub fn random_bounded(min: f32, max: f32) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min, max),
            y: rand::thread_rng().gen_range(min, max),
            z: rand::thread_rng().gen_range(min, max),
        }
    }

    pub fn length_sqrd(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_sqrd().sqrt()
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_bounded(-1.0, 1.0);
        if p.length_sqrd() >= 1.0 { continue };
        return p
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a = rand::thread_rng().gen_range(0.0, 2.0 * PI) as f32;
    let z = rand::thread_rng().gen_range(-1.0, 1.0) as f32;
    let r = (1.0 - z.powi(2)).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&-*uv, n);
    let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_perp = -(1.0 - r_out_parallel.length_sqrd()).sqrt() * *n;
    r_out_parallel + r_out_perp
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::thread_rng().gen_range(-1.0, 1.0), rand::thread_rng().gen_range(-1.0, 1.0), 0.0);
        if p.length_sqrd() >= 1.0 { continue };
        return p
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y, 
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    v / len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negate() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Vec3::new(-1.0, -1.0, -1.0), -vec);
    }

    #[test]
    fn test_add_assign() {
        let mut vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);
        vec1 += vec2;

        let vec3 = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(vec3, vec1);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = Vec3::new(3.0, 3.0, 3.0);
        v1 *= v2;

        let v3 = Vec3::new(6.0, 6.0, 6.0);
        assert_eq!(v3, v1);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        v1 /= v2;

        let v3 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v3, v1);
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);

        let vec3 = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(vec3, vec1 + vec2);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);

        let vec3 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(vec3, vec1 - vec2);
    }

    #[test]
    fn test_div_f32() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);

        let vec3 = Vec3::new(0.5, 0.5, 0.5);
        assert_eq!(vec3, vec1 / 2.0);
    }

    #[test]
    fn test_mul_f32() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);

        let vec3 = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(vec3, vec1 * 3.0);   
    }
}