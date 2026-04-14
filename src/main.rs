use kdam::BarExt;

const IMAGE_W: usize = 256;
const IMAGE_H: usize = 256;

mod canvas;
mod color;

fn main() {
    let mut img = canvas::Canvas::new(IMAGE_W, IMAGE_H);

    let mut bar = img.progress_bar();

    for x in 0..IMAGE_W {
        for y in 0..IMAGE_H {
            let pixel_color =
                color::Color::new(x as f32 / IMAGE_W as f32, y as f32 / IMAGE_H as f32, 0.0);
            img[(x, y)] = pixel_color;
            bar.update(1).unwrap();
        }
    }

    img.write();
}
