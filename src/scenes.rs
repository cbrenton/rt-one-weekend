use std::sync::Arc;

use glam::{DVec3, IVec3};

use crate::{
    camera::CameraConfig,
    geom::{Hittable, HittableList, Sphere, Triangle, TriangleMesh},
    util::{
        CheckerTexture, Color, Dielectric, DiffuseLight, Lambertian, Material, Metal, SolidColor,
    },
};

pub struct SceneData {
    pub world: HittableList,
    pub config: CameraConfig,
    pub name: String,
}

pub fn sample() -> SceneData {
    let config = CameraConfig {
        samples_per_pixel: 400,
        look_from: DVec3::ZERO,
        look_at: DVec3::new(0.0, 0.0, -1.0),
        up: DVec3::new(0.0, 1.0, 0.0),
        vfov: 90.0,
        defocus_angle: 1.0,
        focus_distance: 1.4,
        ..Default::default()
    };
    let mut world = HittableList::default();

    let _material_ground = Arc::new(Lambertian::from_color(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
    let _material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.01));
    let _material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));
    let material_glass = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let light_tex = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    let material_emit = Arc::new(DiffuseLight::new(light_tex));

    // create a fake "plane" via 2 20x20 triangles
    let plane_a = DVec3::new(-10.0, -0.5, -11.0);
    let plane_b = DVec3::new(-10.0, -0.5, 9.0);
    let plane_c = DVec3::new(10.0, -0.5, -11.0);
    let plane_d = DVec3::new(10.0, -0.5, 9.0);
    let checker_tex = Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let material_checker = Arc::new(Lambertian::new(checker_tex));
    let plane_left = Triangle::new(plane_a, plane_b, plane_c, material_checker.clone());
    let plane_right = Triangle::new(plane_b, plane_c, plane_d, material_checker.clone());
    world.add(plane_left);
    world.add(plane_right);

    // world.add(Sphere::new(DVec3::new(1.0, 0.0, -1.2), 0.5, material_right));
    world.add(Sphere::new(DVec3::new(1.0, 0.0, -1.2), 0.5, material_emit));
    world.add(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_glass,
    ));
    world.add(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
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

    SceneData {
        world,
        config,
        name: String::from("sample"),
    }
}

pub fn cornell_box() -> SceneData {
    let config = CameraConfig {
        samples_per_pixel: 1000,
        look_from: DVec3::new(278.0, 278.0, -800.0),
        look_at: DVec3::new(278.0, 278.0, 0.0),
        up: DVec3::new(0.0, 1.0, 0.0),
        vfov: 40.0,
        defocus_angle: 0.0,
        //focus_distance: 1.4,
        aspect_ratio: 1.0,
        ..Default::default()
    };
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(quad_mesh(
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(quad_mesh(
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(quad_mesh(
        DVec3::new(343.0, 554.0, 332.0),
        DVec3::new(-130.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(quad_mesh(
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(quad_mesh(
        DVec3::new(555.0, 555.0, 555.0),
        DVec3::new(-555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(quad_mesh(
        DVec3::new(0.0, 0.0, 555.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        white,
    ));

    SceneData {
        world,
        config,
        name: String::from("cornell_box"),
    }
}

pub fn spooky() -> SceneData {
    let config = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 2000.0,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        look_from: DVec3::new(26.0, 3.0, 6.0),
        look_at: DVec3::new(0.0, 2.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    };
    let mut world = HittableList::default();

    world.add(Sphere::new(
        DVec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_color(Color::new(0.3, 0.3, 0.3))),
    ));
    world.add(Sphere::new(
        DVec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_color(Color::new(0.3, 0.3, 0.3))),
    ));

    let diff_light = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(quad_mesh(
        DVec3::new(3.0, 1.0, -2.0),
        DVec3::new(2.0, 0.0, 0.0),
        DVec3::new(0.0, 2.0, 0.0),
        diff_light,
    ));

    SceneData {
        world,
        config,
        name: String::from("spooky"),
    }
}

// only here until I add quads/boxes or make a real mesh
fn quad_mesh(p: DVec3, u: DVec3, v: DVec3, mat: Arc<dyn Material>) -> TriangleMesh {
    let vertices = vec![p, p + u, p + v, p + u + v];
    let triangles = vec![IVec3::new(0, 1, 2), IVec3::new(1, 3, 2)];
    TriangleMesh::new(vertices, triangles, false, mat)
}
