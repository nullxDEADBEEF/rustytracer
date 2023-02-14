use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Scatter;

#[derive(Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz: {
                if fuzz < 1.0 {
                    fuzz
                } else {
                    1.0
                }
            },
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir.unit().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.dir.dot(rec.normal) > 0.0
    }
}
