use raytracing_weekend_rust::camera::Camera;
use raytracing_weekend_rust::hittable::{Hittable};
use raytracing_weekend_rust::hittable_list::HittableList;
use raytracing_weekend_rust::png;
use raytracing_weekend_rust::ppm;
use raytracing_weekend_rust::point3::Point3;
use raytracing_weekend_rust::sphere::Sphere;
use raytracing_weekend_rust::ray::Ray;
use raytracing_weekend_rust::color::*;
use raytracing_weekend_rust::vec3::Vec3;
use std::path::Path;
use rand::prelude::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(r, 0.0, f64::INFINITY) {
        let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(hit_record.p, target - hit_record.p), world, depth - 1);
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut data: Vec<u8> = vec![];

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r  = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            push_color(&mut data, &pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    png::write_png(Path::new("./result.png"), IMAGE_WIDTH, IMAGE_HEIGHT, data.as_slice());
    ppm::write_ppm(Path::new("./result.ppm"), IMAGE_WIDTH, IMAGE_HEIGHT, data.as_slice());
}
