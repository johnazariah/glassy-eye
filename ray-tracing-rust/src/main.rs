// mod vec;
// use vec::*;
use std::io::Write;

mod vec3d;
mod color;
mod image;

fn main() {
    let mut buffer = std::io::BufWriter::new(std::fs::File::create("hello_world.ppm").unwrap());
    let image = image::Image::<512,512>::generate_red_green_scan();
    write!(buffer, "{}", image).unwrap();
}