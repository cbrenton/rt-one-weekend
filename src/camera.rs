use crate::util::{Canvas, Color, Interval, Ray};
use crate::geom::{Hittable, HittableList, HitRecord};
use glam::Vec3;
use kdam::BarExt;

pub struct Camera {
    pub img: Canvas,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        // make sure height is at least 1
        let image_h = (Self::IMAGE_W / Self::ASPECT_RATIO).max(1.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        // recalculate aspect ratio because image_h might not be what we intended
        let viewport_width = viewport_height * (Self::IMAGE_W / image_h);
        let camera_center = Vec3::ZERO;

        // calculate the vectors along the horizontal and vertical edges of the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / Self::IMAGE_W;
        let pixel_delta_v = viewport_v / image_h;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let img = Canvas::new(Self::IMAGE_W as usize, image_h as usize);

        Self { img, camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    pub fn render(&mut self, world: &HittableList) {
        let mut bar = self.img.progress_bar();
        for x in 0..self.img.width {
            for y in 0..self.img.height {
                let pixel_center =
                    self.pixel00_loc + (x as f32 * self.pixel_delta_u) + (y as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;

                let ray = Ray::new(self.camera_center, ray_direction);

                let pixel_color = self.ray_color(&ray, &world);

                self.img[(x, y)] = pixel_color;
                bar.update(1).unwrap();
            }
        }

        self.img.write();
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut rec) {
            0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction().normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }

    const IMAGE_W: f32 = 800.0;
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
}
