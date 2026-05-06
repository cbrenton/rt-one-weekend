mod camera;
mod geom;
mod scenes;
mod util;

use std::env::args;

use camera::Camera;

fn main() {
    let scene_data = match args().nth(1) {
        Some(arg) => match arg.as_str() {
            "1" => scenes::sample(),
            "2" => scenes::cornell_box(),
            "3" => scenes::spooky(),
            _ => scenes::sample(),
        },
        _ => scenes::sample(),
    };

    let config = scene_data.config;
    let world = scene_data.world;
    println!("rendering scene {}", scene_data.name);

    let mut camera = Camera::new(config);

    // TODO: I don't like how Camera includes image writing - ideally this will get extracted in
    // the future
    camera.render(&world);
}
