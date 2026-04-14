use glam::Vec3;
use kdam::BarExt;

const IMAGE_W: f32 = 800.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;

mod canvas;
mod color;
mod ray;

fn main() {
    let image_h = (IMAGE_W / ASPECT_RATIO) as usize;
    let mut img = canvas::Canvas::new(IMAGE_W as usize, image_h);

    let mut bar = img.progress_bar();
    let r = ray::Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

    for x in 0..img.width {
        for y in 0..img.height {
            let pixel_color = color::Color::new(x as f32 / IMAGE_W, y as f32 / image_h as f32, 0.0);
            img[(x, y)] = pixel_color;
            bar.update(1).unwrap();
        }
    }

    img.write();
}
