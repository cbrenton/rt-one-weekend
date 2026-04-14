use crate::color::Color;
use image::{Rgb, RgbImage};
use kdam::tqdm;
use std::fs;
use std::ops::{Index, IndexMut};

const SCALING_FACTOR: f32 = 255.999;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Color::ZERO; width]; height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn progress_bar(&self) -> kdam::Bar {
        tqdm!(total = self.width * self.height)
    }

    pub fn write(&self) {
        let filename = "output/test.png";

        // create image buffer
        let mut buf = RgbImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let color = self.pixels[y][x];

                // translate the [0.0, 1.0] component values to the byte range [0.0, 255.0]
                let r = (SCALING_FACTOR * color.x) as u8;
                let g = (SCALING_FACTOR * color.y) as u8;
                let b = (SCALING_FACTOR * color.z) as u8;

                let pixel = buf.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb([r, g, b]);
            }
        }

        // create output dir
        let _ = fs::create_dir("./output");

        // write image
        buf.save("output/test.png").unwrap();
        println!("Wrote file to {filename}");
    }
}

// trait allowing the struct to be accessed via img[(x, y)]
impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Color {
        &self.pixels[y][x]
    }
}

// same for mut
impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        &mut self.pixels[y][x]
    }
}
