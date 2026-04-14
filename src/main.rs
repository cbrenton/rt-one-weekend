use image::{Rgb, RgbImage};
use std::fs;

const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;

fn main() {
    let filename = "output/test.png";

    let mut img = RgbImage::new(IMAGE_W, IMAGE_H);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x as u8, y as u8, 0]);
    }

    fs::create_dir("./output");
    img.save("output/test.png");

    println!("Wrote file to {filename}");
}
