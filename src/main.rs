mod camera;
mod geom;
mod util;

use std::sync::Arc;

use camera::Camera;
use geom::{HittableList, Sphere};
use glam::{DVec3, IVec3};

use crate::{
    geom::{Hittable, Plane, Triangle, TriangleMesh},
    util::{Color, Lambertian, Metal},
};

fn main() {
    let mut world = HittableList::default();

    let mut camera = Camera::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.01));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));

    world.add(Plane::new(
        DVec3::new(0.0, -0.5, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
        material_ground,
    ));
    world.add(Sphere::new(DVec3::new(1.0, 0.0, -1.2), 0.5, material_right));
    world.add(Sphere::new(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let a = DVec3::new(-0.7, 0.5, -1.2);
    let b = DVec3::new(0.7, 0.5, -1.2);
    let c = DVec3::new(0.0, -0.5, -1.2);
    let d = DVec3::new(0.0, 0.0, -0.7);
    let vertices = vec![a, b, c, d];
    let triangles = vec![
        IVec3::new(0, 2, 3),
        IVec3::new(2, 1, 3),
        IVec3::new(1, 0, 3),
    ];
    let mesh = TriangleMesh::new(vertices, triangles, false, material_center);
    mesh.debug();
    world.add(mesh);

    // TODO: I don't like how Camera includes image writing - ideally this will get extracted in
    // the future
    camera.render(&world);
}
