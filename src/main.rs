use std::{f32::consts::PI, sync::Arc, time::SystemTime};

use nalgebra::Vector3;
use world::{Disk, World};

mod camera;
mod masses;
mod photon;
mod world;

const NUM_THREADS: usize = 9;
const RES_WIDTH: u32 = 1280;
const RES_HEIGHT: u32 = 720;
const FRAMES: usize = 10;

fn main() {
    let mut world = World::new();

    world.add_mass(Vector3::new(0.0, 0.0, 0.0), 1.0);
    world.add_object(Arc::new(Disk {
        pos: Vector3::new(0.0, 0.0, 0.0),
        inner_rad: 1.0,
        outer_rad: 8.0,
        height: 0.1,
        col: [1.0, 0.5, 0.3],
    }));

    for i in 0..FRAMES {
        let now = SystemTime::now();

        let t = PI * i as f32 / FRAMES as f32;
        world.add_camera(
            Vector3::new(6.0, 5.0, 3.0 * t.cos()),
            Vector3::new(0.0, 0.0, 0.0),
            15.0 * RES_WIDTH as f32 / RES_HEIGHT as f32,
            15.0,
            RES_WIDTH,
            RES_HEIGHT,
        );
        world.split_and_par_render(NUM_THREADS, &format!("out/frame_{i:0>6}.png"));
        world.clear_cameras();

        println!("Saved frame {i} after {:?}.", now.elapsed().unwrap());
    }
}
