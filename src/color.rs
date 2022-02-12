use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn push_color(buffer: &mut Vec<u8>, color: &Color, samples_per_pixel: u32) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let ur = (r * 255.999) as u8;
    let ug = (g * 255.999) as u8;
    let ub = (b * 255.999) as u8;

    buffer.push(ur);
    buffer.push(ug);
    buffer.push(ub);
}