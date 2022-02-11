use raytracing_weekend_rust::png;
use raytracing_weekend_rust::vec3::Vec3;
use raytracing_weekend_rust::point3::*;
use raytracing_weekend_rust::ray::Ray;
use raytracing_weekend_rust::color::*;
use std::path::Path;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let image_path = Path::new("./result.png");

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut data: Vec<u8> = vec![];

    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r  = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);
            push_color(&mut data, &pixel_color);
        }
    }

    png::write_png(image_path, IMAGE_WIDTH, IMAGE_HEIGHT, data.as_slice());
}
