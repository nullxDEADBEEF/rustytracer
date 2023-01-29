use crate::hittable::{Hit, HitRecord};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hit>>,
}

impl HittableList {
    pub fn new(object: Box<dyn Hit>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object)
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
