mod camera;
mod geom;
mod util;

use camera::Camera;
use geom::{HittableList, Sphere};
use glam::Vec3;

fn main() {
    let mut world = HittableList::default();

    let mut camera = Camera::new();

    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // TODO: I don't like how Camera includes image writing - ideally this will get extracted in
    // the future
    camera.render(&world);
}
