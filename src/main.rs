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

use crate::{hittable_list::HittableList, vec3::Point3};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    // the distance between the projection plane and the projection point
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width.into(), 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height.into(), 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT - 1).rev() {
        eprintln!("\rScanlines remaning: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}
