use crate::{
    geom::HitRecord,
    util::{Color, Ray},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> bool;
}

#[derive(Default, Copy, Clone)]
pub struct NoMaterial {}

impl Material for NoMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> bool {
        false
    }
}
