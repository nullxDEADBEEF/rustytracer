use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use self::{lambertian::Lambertian, metal::Metal};

pub mod lambertian;
pub mod metal;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(lambertian) => {
                lambertian.scatter(r_in, rec, attenuation, scattered)
            }
            Material::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Default for Material {
    fn default() -> Self {
        Self::Metal(Metal::new(Vec3::default(), 1.0))
    }
}
