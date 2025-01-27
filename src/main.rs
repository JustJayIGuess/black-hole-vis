use std::{f32::consts::PI, sync::Arc, time::SystemTime};

use nalgebra::Vector3;
use world::{Disk, World};

mod camera;
mod masses;
mod photon;
mod world;

const NUM_THREADS: usize = 9;
const RES_WIDTH: u32 = 2560;
const RES_HEIGHT: u32 = 1440;
const FRAMES: usize = 210;

fn main() {
    let mut world = World::new();

    for i in 0..FRAMES {
        let now = SystemTime::now();

        let t = i as f32 / FRAMES as f32;

        world.add_object(Arc::new(Disk {
            pos: Vector3::new(0.0, 0.0, 0.0),
            inner_rad: 1.0,
            outer_rad: 8.0,
            height: 0.1,
            col: [1.0, 0.5, 0.3],
        }));

        world.add_mass(Vector3::new(0.0, 0.0, 0.0), 1.0);

        world.add_camera(
            Vector3::new(
                9.0 * (2.0 * PI * t).cos(),
                9.0 * (2.0 * PI * t).sin(),
                3.0 * (1.0 / (1.0 + (20.0 * (t - 0.5)).exp()) - 0.5),
            ),
            Vector3::new(0.0, 0.0, 0.0),
            15.0 * RES_WIDTH as f32 / RES_HEIGHT as f32,
            15.0,
            RES_WIDTH,
            RES_HEIGHT,
        );

        world.split_and_par_render(NUM_THREADS, &format!("out/frame_{i:0>6}.png"));
        world.clear_objects();
        world.clear_cameras();
        world.clear_masses();

        println!("Saved frame {i} after {:?}.", now.elapsed().unwrap());
    }
}
