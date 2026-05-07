use std::sync::Arc;

use glam::DVec3;

use crate::{
    geom::HitRecord,
    util::{Color, Ray, SolidColor, Texture, near_zero, random_double, random_unit_vector},
};

pub struct ScatterData {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: DVec3) -> Color {
        Color::ZERO
    }
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        // diffuse reflectance - ray gets scattered in a random dir from the normal
        let mut scatter_dir = rec.normal + random_unit_vector();
        if near_zero(scatter_dir) {
            scatter_dir = rec.normal
        }

        let result = ScatterData {
            attenuation: self.tex.value(rec.u, rec.v, rec.point),
            scattered: Ray::new(rec.point, scatter_dir),
        };
        Some(result)
    }
}

#[derive(Default, Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        // perfect reflectance - ray gets reflected about the normal
        let mut reflected = ray_in.direction().reflect(rec.normal);
        reflected = reflected.normalize() + (self.fuzziness * random_unit_vector());

        let result = ScatterData {
            attenuation: self.albedo,
            scattered: Ray::new(rec.point, reflected),
        };
        if result.scattered.direction().dot(rec.normal) > 0.0 {
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    // TODO: is "ratio" the right term here?
    fn schlick_approx(&self, cosine: f64, refraction_ratio: f64) -> f64 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterData> {
        let attenuation = Color::ONE;
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let direction: DVec3;
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || self.schlick_approx(cos_theta, refraction_ratio) > random_double() {
            // must reflect
            direction = ray_in.direction().reflect(rec.normal);
        } else {
            // can refract
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        }

        let result = ScatterData {
            attenuation: attenuation,
            scattered: Ray::new(rec.point, direction),
        };
        Some(result)
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    pub tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: DVec3) -> Color {
        self.tex.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct NullMaterial {}

impl NullMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for NullMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterData> {
        None
    }
}

pub fn null_material_ptr() -> Arc<NullMaterial> {
    Arc::new(NullMaterial::new())
}
