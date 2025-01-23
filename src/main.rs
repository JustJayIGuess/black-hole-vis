use std::rc::Rc;

use masses::StaticMasses;
use nalgebra::Vector3;
use world::{Sphere, World};

mod masses;
mod photon;
mod world;

fn main() {
    let mut masses = StaticMasses::new();
    masses.add(Vector3::new(1.0, 1.0, 1.0), 1.0);

    let mut world = World::new(
        Vector3::new(-1.0, -1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Rc::new(masses),
    );

    world.add_object(Rc::new(Sphere {
        pos: Vector3::new(0.0, 0.0, 0.0),
        rad: 0.2,
    }));

    world.render();
}
