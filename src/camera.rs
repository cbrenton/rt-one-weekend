use crate::geom::{HitRecord, Hittable, HittableList};
use crate::util::{ALMOST_ZERO, Canvas, Color, Interval, Ray, random_double, random_in_unit_disk};
use glam::DVec3;
use kdam::BarExt;

pub struct CameraConfig {
    // TODO: should probably be called horizontal_resolution
    pub image_width: f64,
    pub aspect_ratio: f64,
    // TODO: these two should probably go in a separate RenderConfig
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: DVec3,
    pub look_at: DVec3,
    pub up: DVec3,
    pub defocus_angle: f64,
    pub focus_distance: f64, // distance from camera look_from point to plane of perfect focus
    pub background: Color,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            image_width: 800.0,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            max_depth: 10,
            vfov: 90.0,
            look_from: DVec3::ZERO,
            look_at: DVec3::new(0.0, 0.0, -1.0),
            up: DVec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            background: Color::ZERO,
        }
    }
}

pub struct Camera {
    config: CameraConfig,
    img: Canvas,
    camera_center: DVec3,
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
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
        let camera_center = config.look_from;

        let theta = config.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * config.focus_distance;
        // recalculate aspect ratio because image_h might not be what we intended
        let viewport_width = viewport_height * (config.image_width / image_h);

        // calculate u,v,w unit basis vectors for camera
        let w = (config.look_from - config.look_at).normalize();
        let u = config.up.cross(w).normalize();
        let v = w.cross(u);

        // calculate the vectors along the horizontal and vertical edges of the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / config.image_width;
        let pixel_delta_v = viewport_v / image_h;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            camera_center - (config.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            config.focus_distance * (config.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let img = Canvas::new(config.image_width as usize, image_h as usize);

        Self {
            config,
            img,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
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

        let ray_origin = if self.config.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_disk();
        self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth >= self.config.max_depth {
            return DVec3::ZERO;
        }

        if let Some(rec) = world.hit(ray, Interval::new(ALMOST_ZERO, f64::INFINITY)) {
            if let Some(mat) = rec.mat.clone() {
                let color_from_emission = mat.emitted(rec.u, rec.v, rec.point);
                if let Some(scatter) = mat.scatter(ray, &rec) {
                    let color_from_scatter =
                        scatter.attenuation * self.ray_color(&scatter.scattered, world, depth + 1);
                    color_from_emission + color_from_scatter
                } else {
                    color_from_emission
                }
            } else {
                Color::ZERO
            }
        } else {
            self.config.background
        }
    }
}
