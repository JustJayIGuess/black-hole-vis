use std::rc::Rc;

use nalgebra::Vector3;

use crate::StaticMasses;

pub struct Photon {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Photon {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32> ) -> Photon {
        Photon {
            pos,
            dir: dir.normalize(),
        }
    }
}

pub trait Physics {
    fn pos(&self) -> &Vector3<f32>;
    fn step(&mut self, environment: Rc<StaticMasses>, step_size: f32);
}

impl Physics for Photon {
    fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    fn step(&mut self, environment: Rc<StaticMasses>, step_size: f32) {
        // self.dir.normalize_mut();
        self.pos = self.pos + self.dir * step_size;
    }
}
