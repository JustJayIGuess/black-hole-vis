use std::rc::Rc;

use nalgebra::Vector3;

use crate::{
    masses::StaticMasses,
    photon::{Photon, Physics},
};

pub struct World {
    camera: Camera,
    objects: Vec<Rc<dyn Visible>>,
    masses: Rc<StaticMasses>,
}

pub trait Visible {
    fn overlap(&self, point: &Vector3<f32>) -> bool;
}

pub struct Sphere {
    pub pos: Vector3<f32>,
    pub rad: f32,
}

impl Visible for Sphere {
    fn overlap(&self, point: &Vector3<f32>) -> bool {
        point.metric_distance(&self.pos) < self.rad
    }
}

pub struct Camera {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

impl World {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>, masses: Rc<StaticMasses>) -> World {
        World {
            camera: Camera { pos, dir },
            objects: vec![],
            masses: masses,
        }
    }

    pub fn add_object(&mut self, object: Rc<dyn Visible>) {
        self.objects.push(object);
    }

    pub fn render(&self) {
        let mut photon = Photon::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            400.0,
        );

        println!("Rendering...");
    }
}
