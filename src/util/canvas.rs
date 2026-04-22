use crate::util::color::linear_to_gamma;
use crate::util::{Color, Interval};
use image::{Rgb, RgbImage};
use kdam::tqdm;
use std::fs;
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::ZERO; width * height];
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
        let dirname = "./output";
        let filename = "test.png";

        // create image buffer
        let mut buf = RgbImage::new(self.width as u32, self.height as u32);
        let intensity = Interval::new(0, 255);
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x];

                // translate the [0.0, 1.0] component values to the byte range [0.0, 255.0]
                let r = intensity.scale(linear_to_gamma(color.x)) as u8;
                let g = intensity.scale(linear_to_gamma(color.y)) as u8;
                let b = intensity.scale(linear_to_gamma(color.z)) as u8;

                let pixel = buf.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb([r, g, b]);
            }
        }

        // create output dir
        let _ = fs::create_dir(dirname);

        // write image
        buf.save(format!("{dirname}/{filename}")).unwrap();
        println!("Wrote file to {filename}");
    }
}

// trait allowing the struct to be accessed via img[(x, y)]
impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Color {
        &self.pixels[y * self.width + x]
    }
}

// same for mut
impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }
}
