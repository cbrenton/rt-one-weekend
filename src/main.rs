use std::mem::Discriminant;

use glam::Vec3;
use kdam::BarExt;

const IMAGE_W: f32 = 800.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;

mod canvas;
mod color;
mod ray;

fn ray_color(r: ray::Ray) -> color::Color {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let normal = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * color::Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * color::Color::new(1.0, 1.0, 1.0) + a * color::Color::new(0.5, 0.7, 1.0)
}

// return the value of t where the ray intersects with sphere (or -1 if it doesn't)
fn hit_sphere(center: Vec3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = center - r.origin();

    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = h.powi(2) - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn main() {
    // make sure height is at least 1
    let image_h = (IMAGE_W / ASPECT_RATIO).max(1.0);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    // recalculate aspect ratio because image_h might not be what we intended
    let viewport_width = viewport_height * (IMAGE_W / image_h);
    let camera_center = Vec3::ZERO;

    // calculate the vectors along the horizontal and vertical edges of the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / IMAGE_W;
    let pixel_delta_v = viewport_v / image_h;

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut img = canvas::Canvas::new(IMAGE_W as usize, image_h as usize);
    let mut bar = img.progress_bar();

    let r = ray::Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

    for x in 0..img.width {
        for y in 0..img.height {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);

            // let pixel_color = color::Color::new(x as f32 / IMAGE_W, y as f32 / image_h, 0.0);
            img[(x, y)] = pixel_color;
            bar.update(1).unwrap();
        }
    }

    img.write();
}
