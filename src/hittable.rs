use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // if dot product is positive we are inside sphere
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
