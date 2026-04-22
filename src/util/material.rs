use crate::{
    geom::HitRecord,
    util::{Color, Ray, near_zero, random_unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Default, Copy, Clone)]
pub struct NoMaterial {}

impl Material for NoMaterial {
    fn scatter(&self, _: &Ray, _: &mut HitRecord, _: &mut Color, _: &mut Ray) -> bool {
        false
    }
}

#[derive(Default, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // diffuse reflectance - ray gets scattered in a random dir from the normal
        let mut scatter_dir = rec.normal + random_unit_vector();
        if near_zero(scatter_dir) {
            scatter_dir = rec.normal
        }

        *scattered = Ray::new(rec.point, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Default, Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // perfect reflectance - ray gets reflected about the normal
        let mut reflected = ray_in.direction().reflect(rec.normal);
        reflected = reflected.normalize() + (self.fuzziness * random_unit_vector());

        *scattered = Ray::new(rec.point, reflected);
        *attenuation = self.albedo;
        true
    }
}
