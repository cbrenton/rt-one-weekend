use image::{Rgb, RgbImage};
use std::fs;

const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;

fn main() {
    let filename = "output/test.png";

    let mut img = RgbImage::new(IMAGE_W, IMAGE_H);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }

    fs::create_dir("./output");
    img.save("output/test.png");

    println!("Wrote file to {filename}");
}
