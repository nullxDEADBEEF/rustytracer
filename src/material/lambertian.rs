use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Scatter;

#[derive(Default, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction =
            rec.normal + Vec3::random_unit_vector(Vec3::random_in_unit_sphere());

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
