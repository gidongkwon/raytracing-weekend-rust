use std::{path::Path, fs::File, io::{BufWriter, Write}};

pub fn write_ppm(path: &Path, width: u32, height: u32, data: &[u8]) {
    let file = File::create(path).unwrap();
    let mut w = BufWriter::new(file);

    let header = format!("P3\n{} {}\n255\n", width, height);
    w.write(header.as_bytes()).unwrap();

    for (index, color) in data.iter().enumerate() {
        w.write(color.to_string().as_bytes()).unwrap();
        if (index + 1) % 3 == 0 {
            w.write(b"\n").unwrap();
        } else {
            w.write(b" ").unwrap();
        }
    }
}