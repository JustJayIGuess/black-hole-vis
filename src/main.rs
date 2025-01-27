use std::{sync::Arc, time::SystemTime};

use nalgebra::Vector3;
use world::{Disk, World};

mod camera;
mod masses;
mod photon;
mod world;

const NUM_THREADS: usize = 10;

fn main() {
    let mut world = World::new();

    world.add_camera(
        Vector3::new(6.0, 5.0, 1.5),
        Vector3::new(0.0, 0.0, 0.0),
        15.0,
        15.0,
        1024,
        1024,
    );

    world.split_camera(0, NUM_THREADS);

    world.add_mass(Vector3::new(0.0, 0.0, 0.0), 1.0);

    world.add_object(Arc::new(Disk {
        pos: Vector3::new(0.0, 0.0, 0.0),
        inner_rad: 1.0,
        outer_rad: 8.0,
        height: 0.05,
        col: [1.0, 0.5, 0.3],
    }));

    let now = SystemTime::now();
    world.par_render_split_pngs();
    world.stitch_pngs(NUM_THREADS, "out");
    print!("Saved after {:?}.", now.elapsed().unwrap());
}
