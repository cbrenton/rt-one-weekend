use crate::geom::{HitRecord, Hittable, HittableList};
use crate::util::{Canvas, Color, Interval, Ray, random_double};
use glam::DVec3;
use kdam::BarExt;

pub struct CameraConfig {
    pub image_width: f64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            image_width: 800.0,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            max_depth: 10,
        }
    }
}

pub struct Camera {
    config: CameraConfig,
    img: Canvas,
    pub camera_center: DVec3,
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
}

impl Default for Camera {
    fn default() -> Self {
        let config = CameraConfig::default();
        Self::new(config)
    }
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        // make sure height is at least 1
        let image_h = (config.image_width / config.aspect_ratio).max(1.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        // recalculate aspect ratio because image_h might not be what we intended
        let viewport_width = viewport_height * (config.image_width / image_h);
        let camera_center = DVec3::ZERO;

        // calculate the vectors along the horizontal and vertical edges of the viewport
        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / config.image_width;
        let pixel_delta_v = viewport_v / image_h;

        // calculate the location of the upper left pixel
        let viewport_upper_left = camera_center
            - DVec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let img = Canvas::new(config.image_width as usize, image_h as usize);

        Self {
            config,
            img,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        let mut bar = self.img.progress_bar();
        for y in 0..self.img.height {
            for x in 0..self.img.width {
                let mut pixel_color = Color::ZERO;

                // cast SAMPLES_PER_PIXEL random-ish rays and then divide total color by
                // SAMPLES_PER_PIXEL for simple antialiasing
                for _ in 0..self.config.samples_per_pixel {
                    let ray = self.get_ray(x, y);

                    pixel_color += self.ray_color(&ray, world, 0);
                }
                bar.update(1).unwrap();
                self.img[(x, y)] = pixel_color / self.config.samples_per_pixel as f64;
            }
        }

        self.img.write();
    }

    pub fn debug(&mut self, world: &HittableList, ray: &Ray) {
        self.ray_color(&ray, world, 0);
    }

    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = self.sample_square();

        let pixel_center = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_center - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth >= self.config.max_depth {
            return DVec3::ZERO;
        }

        if let Some(rec) = world.hit(ray, Interval::new(1e-6, f64::INFINITY)) {
            if let Some(mat) = rec.mat.clone() {
                if let Some(scatter) = mat.scatter(ray, &rec) {
                    scatter.attenuation * self.ray_color(&scatter.scattered, world, depth + 1)
                } else {
                    Color::ZERO
                }
            } else {
                Color::ZERO
            }
        } else {
            let unit_direction = ray.direction().normalize();

            // calculate how "up" y is, on a scale of 0 to 1 (add 1 to convert -1..1 -> 0..2, then
            // multiply by 0.5 to get 0..1)
            let upness = 0.5 * (unit_direction.y + 1.0);

            let white = Color::new(1.0, 1.0, 1.0);
            let sky_blue = Color::new(0.5, 0.7, 1.0);

            // lerp between white and a light blue, based on how "up" the ray is pointing, to get a
            // white sky when the ray is completely downward-facing or a blue sky when it's
            // completely upward facing (or in between for most sky-intersecting rays from camera)
            (1.0 - upness) * white + upness * sky_blue
        }
    }
}
