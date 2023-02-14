mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

use hittable::{Hit, HitRecord};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

use crate::{camera::Camera, hittable_list::HittableList, util::random_double, vec3::Point3};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    // if we've exceeded ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = r.dir.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u16 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render

    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT - 1).rev() {
        eprintln!("\rScanlines remaning: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL as i32);
        }
    }

    eprintln!("Done.")
}
