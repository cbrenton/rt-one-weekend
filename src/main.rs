mod camera;
mod geom;
mod util;

use std::sync::Arc;

use camera::Camera;
use geom::{HittableList, Sphere};
use glam::DVec3;

use crate::util::{Color, Lambertian, Metal};

fn main() {
    let mut world = HittableList::default();

    let mut camera = Camera::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        DVec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // TODO: I don't like how Camera includes image writing - ideally this will get extracted in
    // the future
    camera.render(&world);
}
