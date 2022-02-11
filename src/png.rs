use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn write_png(path: &Path, width: u32, height: u32, data: &[u8]) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}
