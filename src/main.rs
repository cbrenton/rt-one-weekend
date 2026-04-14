use image::{Rgb, RgbImage};
use kdam::{BarExt, tqdm};
use std::fs;

const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;

fn main() {
    let filename = "output/test.png";

    let mut img = RgbImage::new(IMAGE_W, IMAGE_H);

    let mut bar = tqdm!(total = IMAGE_W as usize * IMAGE_H as usize);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x as u8, y as u8, 0]);
        bar.update(1).unwrap();
    }

    // create output dir
    fs::create_dir("./output");

    // write image
    img.save("output/test.png");
    println!("Wrote file to {filename}");
}
