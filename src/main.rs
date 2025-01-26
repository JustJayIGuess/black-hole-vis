use std::rc::Rc;

use masses::StaticMasses;
use nalgebra::Vector3;
use world::{Sphere, TestBlobs, World};

mod camera;
mod masses;
mod photon;
mod world;

fn main() {
    let mut masses = StaticMasses::new();
    // masses.add(Vector3::new(1.0, 1.0, 1.0), 1.0);

    let mut world = World::new(
        Vector3::new(-6.0, 3.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        15.0,
        15.0,
        512,
        512,
        Rc::new(masses),
    );

    // world.add_object(Rc::new(Sphere {
    //     pos: Vector3::new(0.0, 0.0, 0.0),
    //     rad: 4.0,
    // }));
    world.add_object(Rc::new(
        TestBlobs {
            pos: Vector3::new(0.0,0.0, 0.0),
            scale: 4.0,
            size: 2.0,
        }
    ));

    world.render();
}
