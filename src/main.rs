mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use hittable::{Hit, HitRecord};
use ray::Ray;
use sphere::Sphere;
use vec3::Color;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::{lambertian::Lambertian, metal::Metal, Material},
    util::random_double,
    vec3::Point3,
};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    // if we've exceeded ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
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
    let material_ground = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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
