use std::{sync::Arc, time::SystemTime};

use black_hole_vis::camera::Ortho;
use black_hole_vis::objects::Disk;
use black_hole_vis::world::World;
use black_hole_vis::masses::StaticMass;
use glam::vec3;

const NUM_THREADS: usize = 10;
const RES_WIDTH: u32 = 1920;
const RES_HEIGHT: u32 = 1080;

#[allow(clippy::cast_precision_loss)]
fn main() {
    let mut world = World::default();

    let now = SystemTime::now();

    world.add_object(Arc::new(Disk {
        pos: vec3(0.0, 0.0, 0.0),
        inner_rad: 1.0,
        outer_rad: 8.0,
        height: 0.1,
        col: [1.0, 0.5, 0.3],
    }));

    world.add_mass(StaticMass {
        pos: vec3(0.0, 0.0, 0.0),
        mass: 10.0,
    });

    world.add_camera(Ortho::new(
        vec3(
            8.0,
            0.0,
            2.0,
        ),
        vec3(0.0, 0.0, 1.0),
        15.0 * RES_WIDTH as f32 / RES_HEIGHT as f32,
        15.0,
        RES_WIDTH,
        RES_HEIGHT,
        0.55,
    ));

    world.split_and_par_render(NUM_THREADS, &format!("wallpaper.png"));

    println!("Saved picture after {:?}.", now.elapsed().unwrap());
}
