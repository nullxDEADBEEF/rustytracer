use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::util::{clamp, random_double, random_double_param};

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        // Divide the color by the number of samples and gamme-correct for gamma=2.0
        // we have to gamma-correct since many image viewers assume the image is gamma-corrected
        // meaning the 0 -> 1 values have some transform before being stored as a byte
        // for now we are doing 50% color reflectors
        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        // Write translated [0,255] value of each color component
        println!(
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as i32,
            (256.0 * clamp(g, 0.0, 0.999)) as i32,
            (256.0 * clamp(b, 0.0, 0.999)) as i32
        );
    }
}

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    // TODO: This can probably be refactored
    pub fn unit(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    #[inline]
    pub fn random_min_max(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_param(min, max),
            random_double_param(min, max),
            random_double_param(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_min_max(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector(vec: Vec3) -> Vec3 {
        vec.unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            // in the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    // returns true if the vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    pub fn reflect(&self, rhs: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(*rhs) * *rhs
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        // NOTE: do I have to point to self? *self
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        // NOTE: do I have to point to self? *self
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ToString for Vec3 {
    #[inline]
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}
