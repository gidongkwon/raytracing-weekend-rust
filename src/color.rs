use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn push_color(buffer: &mut Vec<u8>, color: &Color) {
    let ur = (color.x * 255.999) as u8;
    let ug = (color.y * 255.999) as u8;
    let ub = (color.z * 255.999) as u8;

    buffer.push(ur);
    buffer.push(ug);
    buffer.push(ub);
}