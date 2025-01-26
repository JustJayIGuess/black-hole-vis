use std::rc::Rc;

use masses::StaticMass;
use nalgebra::Vector3;
use world::{Sphere, TestBlobs, World};

mod camera;
mod masses;
mod photon;
mod world;

fn main() {
    let masses = vec![
        StaticMass { pos: Vector3::new(2.0, 7.0, 0.0), mass: 1.0 }
    ];

    let mut world = World::new(
        Vector3::new(6.0, 5.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        20.0,
        20.0,
        1024,
        1024,
        Rc::new(masses),
    );

    world.add_object(Rc::new(Sphere {
        pos: Vector3::new(3.0, 0.0, 2.0),
        rad: 3.0,
        col: [0.8, 0.6, 0.9]
    }));
    world.add_object(Rc::new(TestBlobs {
        pos: Vector3::new(0.0, 0.0, 0.0),
        scale: 4.0,
        size: 1.7,
        col: [0.9, 0.7, 0.5]
    }));


    world.render("out.png");
}
